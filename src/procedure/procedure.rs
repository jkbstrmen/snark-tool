use crate::graph::undirected::UndirectedGraph;
use crate::procedure::error::Error;
use std::collections::HashMap;
use std::result;

pub type Result<T> = result::Result<T, Error>;
pub type GraphProperties = HashMap<String, serde_json::Value>;

pub trait Procedure<G: UndirectedGraph> {
    fn run(&self, graphs: &mut Vec<(G, GraphProperties)>) -> Result<()>;
}
