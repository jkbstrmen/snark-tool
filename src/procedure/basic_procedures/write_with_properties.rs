use crate::graph::graph::Graph;
use crate::procedure::procedure::{Procedure, BasicProperties, Result, Config};
use std::marker;
use crate::procedure::procedure_builder::ProcedureBuilder;

struct WriteWithPropertiesProcedure<G> {
    config: WriteWithPropertiesProcedureConfig,
    _ph: marker::PhantomData<G>,
}

struct WriteWithPropertiesProcedureConfig {

}

pub struct WriteWithPropertiesProcedureBuilder {

}

impl<G: Graph> Procedure<G> for WriteWithPropertiesProcedure<G> {
    fn run(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()> {
        unimplemented!()
    }
}

impl<G: Graph> WriteWithPropertiesProcedure<G> {

}

impl WriteWithPropertiesProcedureConfig {

}

impl<G: Graph + 'static> ProcedureBuilder<G> for WriteWithPropertiesProcedureBuilder {
    fn build(&self, config: Config) -> Box<dyn Procedure<G>> {
        unimplemented!()
    }
}

