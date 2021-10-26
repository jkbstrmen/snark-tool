//! # snark-tool
//! Snark-tool contains structures and algorithm for structural analysis of cubic graphs.
//!
//! # Regular colourability
//! One of the most important functions provided by snark-tool is function to resolve regular 3-edge
//! colourability of given graph. Here in version 0.4.0 all implemented colourisers works only for
//! cubic graphs.
//!
//! # Basic usage
//! Example of basic usage of snark-tool.
//!
//! ```
//! use snark_tool::graph::graph::{Graph, GraphConstructor};
//! use snark_tool::graph::undirected::simple_graph::graph::SimpleGraph;
//!
//! fn main() {
//!     let mut graph = SimpleGraph::new();
//!     graph.add_edge(0, 5);
//!     graph.add_edge(2, 4);
//!
//!     for edge in graph.edges() {
//!         println!("{:?}", edge);
//!     }
//! }
//! ```

pub mod graph;
pub mod procedure;
pub mod service;
mod tests;
