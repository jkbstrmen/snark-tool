//! Procedure tools encapsulate snark-tool's functionality to procedures runnable in chain.
//!
//! # Basic usage
//! Creation of basic procedure chain:
//!
//! ```
//! # use snark_tool::procedure::basic_procedures::read::{ReadProcedureConfig, ReadProcedureBuilder};
//! # use snark_tool::procedure::basic_procedures::read;
//! # use snark_tool::procedure::basic_procedures::colour::{ColourProcedureConfig, ColourProcedureBuilder};
//! # use snark_tool::procedure::basic_procedures::counter::{CounterProcedureConfig, CounterProcedureBuilder};
//! # use snark_tool::procedure::procedure_chain::ProcedureChain;
//! # use snark_tool::graph::undirected::simple_graph::graph::SimpleGraph;
//! # use snark_tool::procedure::procedure::GraphProperties;
//! #
//! // how to create chain of procedures
//! fn main() {
//!     let read_config = ReadProcedureConfig::new(
//!         "path-to-file.g6".to_string(),
//!         read::G6_FORMAT.to_string(),
//!         None,
//!     );
//!     let read = ReadProcedureBuilder::build(read_config);
//!     let colour_config = ColourProcedureConfig::default();
//!     let colour = ColourProcedureBuilder::build(colour_config);
//!     let counter_config = CounterProcedureConfig::new(true);
//!     let counter = CounterProcedureBuilder::build(counter_config);
//!
//!      let mut procedures = vec![];
//!     procedures.push(read);
//!     procedures.push(colour);
//!     procedures.push(counter);
//!
//!     let chain = ProcedureChain::from_procedures(procedures).unwrap();
//!     let mut graphs_with_properties: Vec<(SimpleGraph, GraphProperties)> = vec![];
//!     let _result = chain.run(&mut graphs_with_properties);
//! }
//! ```

pub mod basic_procedures;
pub mod configuration;
pub mod error;
pub mod helpers;
pub mod procedure;
pub mod procedure_builder;
pub mod procedure_chain;
pub mod procedure_registry;

#[cfg(test)]
mod tests;
