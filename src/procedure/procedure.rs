use crate::error::Error;
use crate::graph::graph::Graph;
use std::collections::HashMap;
use std::result;

pub type Result<T> = result::Result<T, Error>;
pub type BasicProperties = HashMap<String, String>;

// temp
pub type Config = HashMap<String, String>;

pub trait Procedure<G: Graph> {
    fn run(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()>;
}
