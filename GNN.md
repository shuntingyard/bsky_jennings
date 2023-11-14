# Notes on Graph Neural Networks

Intended to process data representable as a graph, they are a whole class of
neural networks with different architectures. Generally speaking they learn
from message passing between nodes.

Application domains are

- social networks (recommender systems etc.)
- computer vision (pixels as nodes, adjacency of pixels as edges)
- NLP (e.g. transformer layers working on complete graphs over a sequence of words)
- chip design (floor planning)
- [NP-hard](https://en.wikipedia.org/wiki/NP-hard)
    [combinatorial optimization](https://en.wikipedia.org/wiki/Combinatorial_optimization)
    problems (related to e.g. chip design)
- computer security for anomaly detection in networks and detection of
    [lateral movement](https://en.wikipedia.org/wiki/Network_Lateral_Movement)
- molecular biology, chemistry, physics

## Links

- [Wikipedia](https://en.wikipedia.org/wiki/Graph_neural_network)
- [PyTorch Geometric](https://arxiv.org/abs/1903.02428)
- [TensorFlow GNN](https://github.com/tensorflow/gnn)
- [Rusty YouTube example](https://youtu.be/OMJ8gFevV38) (to be assessed)
- [caffe2-graph](https://crates.io/crates/caffe2-graph), a crate to look at?
- A graph placement methodology for fast chip design,
    [Nature](https://www.nature.com/articles/s41586-021-03544-w) 2021
