-- In honor of Helen Hall Jennings

CREATE SCHEMA IF NOT EXISTS jenningslab;

-- Representing `app.bsky.graph.get_follows` as a relationship
CREATE TABLE IF NOT EXISTS jenningslab.follow (
    did TEXT NOT NULL,
    first_seen_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_seen_at TIMESTAMP NOT NULL DEFAULT NOW(),
    tid TEXT NOT NULL,

    PRIMARY KEY (did, first_seen_at)
);

-- Representing Bluesky's `actor`s as an entity
CREATE TABLE IF NOT EXISTS jenningslab.actor (
    did TEXT NOT NULL,
    indexed_at TIMESTAMP, -- optional on Bluesky's API
    is_root_node BOOLEAN DEFAULT false,
    has_tombstone BOOLEAN DEFAULT false,
    -- meta_last_traversed_at TIMESTAMP,

    PRIMARY KEY (did)
);

-- The following are attributes of `actor`.

CREATE TABLE IF NOT EXISTS jenningslab.avatar (
    -- Subject to experimentation, as we don't know yet whether
    -- a change of profile pic incurs a change in the URI's value.
    did TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    uri TEXT,
    dhash NUMERIC,

    PRIMARY KEY (did, created_at),
    FOREIGN KEY (did) REFERENCES jenningslab.actor (did) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS jenningslab.description (
    did TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    val TEXT,

    PRIMARY KEY (did, created_at),
    FOREIGN KEY (did) REFERENCES jenningslab.actor (did) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS jenningslab.display_name (
    did TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    val TEXT,

    PRIMARY KEY (did, created_at),
    FOREIGN KEY (did) REFERENCES jenningslab.actor (did) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS jenningslab.handle (
    did TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    val TEXT,

    PRIMARY KEY (did, created_at),
    FOREIGN KEY (did) REFERENCES jenningslab.actor (did) ON DELETE CASCADE
);

-- Meta information helping to measure efficiency and offer
-- insights into progress details of graph traversals.
CREATE TABLE IF NOT EXISTS jenningslab.meta_traversed_from (
    did TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    sid TEXT, -- source id: the node we came from when traversing

    PRIMARY KEY (did, created_at),
    FOREIGN KEY (did) REFERENCES jenningslab.actor (did) ON DELETE CASCADE
);

-- Meta information to determine the validity period of `follow`s
-- as well as state/completion of the last graph traversal.
CREATE TABLE jenningslab.meta (
    _singleton INTEGER DEFAULT 1,
    initial_end TIMESTAMP,
    last_start TIMESTAMP,
    last_end TIMESTAMP,
    curr_start TIMESTAMP

    CHECK (_singleton = 1)
);
