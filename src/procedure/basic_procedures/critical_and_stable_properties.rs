use crate::graph::graph::Graph;
use crate::procedure::procedure::{Procedure, BasicProperties, Result, Config};
use std::marker;
use crate::procedure::procedure_builder::ProcedureBuilder;

struct CriticAndStablePropsProcedure<G> {
    config: CriticAndStablePropsProcedureConfig,
    _ph: marker::PhantomData<G>,
}

struct CriticAndStablePropsProcedureConfig {

}

pub struct CriticAndStablePropsProcedureBuilder {

}

impl<G: Graph> Procedure<G> for CriticAndStablePropsProcedure<G> {
    fn run(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()> {
        unimplemented!()
    }
}

impl<G: Graph> CriticAndStablePropsProcedure<G> {

}

impl CriticAndStablePropsProcedureConfig {

}

impl<G: Graph + 'static> ProcedureBuilder<G> for CriticAndStablePropsProcedureBuilder {
    fn build(&self, config: Config) -> Box<dyn Procedure<G>> {
        unimplemented!()
    }
}
