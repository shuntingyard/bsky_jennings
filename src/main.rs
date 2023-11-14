use std::{collections::HashSet, path::PathBuf, time::Duration};

use async_recursion::async_recursion;
use atrium_api::app::bsky::{actor::get_profile, graph::get_follows};
use clap::Parser;
use tokio::sync::mpsc;
use tracing::{error, info, trace, warn};
use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
};
use url::{Position, Url};
use atp_client::XrpcClient;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    /// A file to store JSON Web Tokens in
    #[clap(short = 'l', long)]
    local_storage: Option<PathBuf>,
    /// Which atproto service to connect to
    #[clap(short = 's', long = "service", env)]
    atp_service: Url,
    /// Username to log in with
    #[clap(short = 'u', long = "username", env)]
    atp_username: String,
    /// Password to log in with
    #[clap(short = 'p', long = "password", env)]
    atp_password: String,
    /// Username to get oldest post for
    #[clap(index = 1)]
    query: Option<String>,
}

#[tokio::main]
async fn main() {
    // Show what's up on RUST_LOG's demand...
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let args = Arguments::parse();

    // Sanitize URL, there mustn't even be a `/` starting the path.
    let url_base = args.atp_service[..Position::AfterPort]
        .to_string()
        .into_boxed_str();

    let client = XrpcClient::login(&url_base, args.atp_username, args.atp_password)
        .await
        .unwrap();
    let profile_client = client.clone();
    let follow_client = client.clone();

    // Setup for reading `actor`s attributes
    let (tx_read_attr, rx_read_attr) = mpsc::channel::<String>(100);
    let profile_handle = tokio::spawn(async move { read_attr(profile_client, rx_read_attr).await });

    //let root = "did:plc:j4fgugpzzggivsd7c3hkksvv"; // <- that's me...
    let root = "toooni.bsky.social";

    // Storage for tracking actions per edge (actor) to prevent expensive things
    // from being done more than once in graph traversal.
    let mut attr_done: HashSet<String> = HashSet::new();
    let mut follows_done: HashSet<String> = HashSet::new();

    // Recursively traverse `follow`s of all actors in graph.
    traverse_recursive(
        follow_client,
        root,
        0,
        &mut attr_done,
        &mut follows_done,
        tx_read_attr,
    )
    .await;

    profile_handle
        .await
        .expect("Profile attributes writer should exit gracefully");
    info!(
        "Final cardinality of actor set: {} ... all done!",
        attr_done.len()
    );
}

async fn read_attr(client: XrpcClient, mut actor_did_rx: mpsc::Receiver<String>) {
    while let Some(actor) = actor_did_rx.recv().await {
        // Read next profile from Bluesky's API.
        let result = client
            .service
            .app
            .bsky
            .actor
            .get_profile(get_profile::Parameters { actor })
            .await;

        match result {
            Err(e) => error!("Failed to retrieve actor for attribute persistence {e}"),
            Ok(output) => {
                eprintln!(
                    "{:?}",
                    (
                        output.did,
                        output.indexed_at,
                        output.display_name,
                        output.followers_count
                    )
                )
            }
        }
    }
}

// #[async_recursion]
#[async_recursion(?Send)]
async fn traverse_recursive(
    client: XrpcClient,
    actor: &str,
    root_distance: u8,
    attr_done: &mut HashSet<String>,
    follows_done: &mut HashSet<String>,
    actor_did_tx: mpsc::Sender<String>,
) {
    // To be done exactly once per edge (actor)
    if attr_done.insert(actor.to_string()) {
        actor_did_tx
            .send(actor.to_string())
            .await
            .unwrap_or_else(|e| error!("Failed to tx '{}', ERROR {}", actor, e));
    }
    // We don't look at `follow`s when there's more than 1 edge to traverse from roots.
    if root_distance > 1 {
        return;
    }
    // Prepare and do cursored retrieval of follows.
    let mut cursor: Option<String> = None;
    let limit: Option<i32> = Some(100);

    loop {
        let result = client
            .service
            .app
            .bsky
            .graph
            .get_follows(get_follows::Parameters {
                actor: actor.to_string(),
                cursor: cursor.clone(),
                limit,
            })
            .await;

        let output = match result {
            Err(e) => panic!("{e}"),
            Ok(out) => out,
        };
        for follow in output.follows {
            // And actually a go at current follows. Let's do pre-traversal:
            println!(" {} -> {}", &output.subject.did, &follow.did);

            // To be done exactly once per edge (actor)
            if follows_done.insert(follow.did.to_string()) {
                warn!(
                    "traversing follows of {}, actor#{:6}",
                    &follow.handle,
                    follows_done.len(),
                );
                traverse_recursive(
                    client.clone(),
                    &follow.did,
                    root_distance + 1,
                    attr_done,
                    follows_done,
                    actor_did_tx.clone(),
                )
                .await;
            }
        }
        cursor = output.cursor;
        if cursor.is_none() {
            break;
        }
    }
}
