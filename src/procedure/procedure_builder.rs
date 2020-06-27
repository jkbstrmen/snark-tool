use crate::graph::graph::Graph;
use crate::procedure::configuration::ProcedureConfig;
use crate::procedure::procedure::{Procedure, Result};
use std::collections::HashMap;

pub type Config = HashMap<String, serde_json::Value>;

pub trait ProcedureBuilder<G: Graph> {
    fn build(&self, config: Config) -> Result<Box<dyn Procedure<G>>>;
}
