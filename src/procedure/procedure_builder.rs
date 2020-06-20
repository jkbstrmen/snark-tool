use crate::graph::graph::Graph;
use crate::procedure::configuration::ProcedureConfig;
use crate::procedure::procedure::{Config, Procedure};

pub trait ProcedureBuilder<G: Graph> {
    // fn build(&self, config: ProcedureConfig) -> Box<dyn Proc<G = G>>;

    fn build(&self, config: Config) -> Box<dyn Procedure<G>>;
}
