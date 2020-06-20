use crate::graph::graph::Graph;
use crate::procedure::procedure::{Procedure, BasicProperties, Result, Config};
use std::marker;
use crate::procedure::procedure_builder::ProcedureBuilder;

struct FilterProcedure<G> {
    config: FilterProcedureConfig,
    _ph: marker::PhantomData<G>,
}

struct FilterProcedureConfig {

}

pub struct FilterProcedureBuilder {

}

impl<G: Graph> Procedure<G> for FilterProcedure<G> {
    fn run(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()> {
        unimplemented!()
    }
}

impl<G: Graph> FilterProcedure<G> {

}

impl FilterProcedureConfig {

}

impl<G: Graph + 'static> ProcedureBuilder<G> for FilterProcedureBuilder {
    fn build(&self, config: Config) -> Box<dyn Procedure<G>> {
        unimplemented!()
    }
}