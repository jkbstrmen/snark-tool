use crate::error::Error;
use crate::graph::graph::Graph;
use crate::procedure::procedure::{BasicProperties, Procedure};
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
    fn run(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()> {
        println!("unknown procedure type: {}", self.proc_type);
        Ok(())
    }
}
