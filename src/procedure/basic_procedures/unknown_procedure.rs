use crate::graph::graph::Graph;
use crate::procedure::error::Error;
use crate::procedure::procedure::{GraphProperties, Procedure};
use std::result;

pub type Result<T> = result::Result<T, Error>;

pub struct UnknownProcedure {
    proc_type: String,
}

impl UnknownProcedure {
    pub fn of_type(proc_type: String) -> Self {
        UnknownProcedure { proc_type }
    }
}

impl<G: Graph> Procedure<G> for UnknownProcedure {
    fn run(&self, _graphs: &mut Vec<(G, GraphProperties)>) -> Result<()> {
        println!("unknown procedure type: {}", self.proc_type);
        Ok(())
    }
}
