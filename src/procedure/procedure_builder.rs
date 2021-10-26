use std::collections::HashMap;

use crate::graph::graph::Graph;
use crate::procedure::procedure::{Procedure, Result};

pub type ConfigMap = HashMap<String, serde_json::Value>;

pub trait ProcedureBuilder<G: Graph> {
    fn build_from_map(&self, config: ConfigMap) -> Result<Box<dyn Procedure<G>>>;
}
