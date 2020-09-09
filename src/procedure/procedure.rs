use crate::error::Error;
use crate::graph::graph::Graph;
use std::collections::HashMap;
use std::result;

pub type Result<T> = result::Result<T, Error>;
pub type GraphProperties = HashMap<String, serde_json::Value>;

pub trait Procedure<G: Graph> {
    fn run(&self, graphs: &mut Vec<(G, GraphProperties)>) -> Result<()>;
}
