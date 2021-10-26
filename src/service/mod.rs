//! Service module contains various functions on graphs.
//!
//! # Basic usage
//! Few examples of usage of available functions.
//! ```
//! use snark_tool::graph::undirected::simple_graph::graph::SimpleGraph;
//! use snark_tool::service::io::reader_g6::G6Reader;
//! use snark_tool::service::colour::recursive::dfs_improved::DFSColourizer;
//! use snark_tool::service::colour::colouriser::Colouriser;
//! use snark_tool::service::chromatic_properties::resistance::Resistance;
//!
//! // read graph from G6 string
//! let petersen_graph_in_g6 = "I?h]@eOWG";
//! let graph: SimpleGraph = G6Reader::read_graph(petersen_graph_in_g6).unwrap();
//!
//! // resolve regular 3-edge colourability of graph
//! let colourable = DFSColourizer::is_colorable(&graph);
//!
//! // resolve resistance of graph
//! let resistance_resolver = Resistance::new_with_colouriser(DFSColourizer::new());
//! let resistance = resistance_resolver.vertex_resistance(&graph);
//! ```

pub mod chromatic_properties;
pub mod colour;
pub mod component_analysis;
pub mod constructions;
pub mod graph_traversal;
pub mod io;
pub mod matching;
pub mod property;
