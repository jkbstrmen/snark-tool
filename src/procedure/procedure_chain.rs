use crate::error::Error;
use crate::graph::edge::Edge;
use crate::graph::graph::{Graph, GraphConstructor};
use crate::graph::vertex::Vertex;
use crate::procedure::configuration::ProcedureConfig;
use crate::procedure::procedure::{BasicProperties, Procedure};
use crate::procedure::procedure_registry::ProcedureRegistry;
use std::collections::HashMap;
use std::{marker, result};

type Result<T> = result::Result<T, Error>;

pub struct ProcedureChain<G: Graph> {
    proc_registry: ProcedureRegistry<G>,
    procedures: Vec<Box<dyn Procedure<G>>>,
}

impl<G: Graph + GraphConstructor + Clone + 'static> ProcedureChain<G> {
    pub fn from_procedures_config(
        registry: ProcedureRegistry<G>,
        configurations: Vec<ProcedureConfig>,
    ) -> Self {
        let mut procedures = vec![];
        for configuration in configurations {
            let proc = registry.create_procedure(configuration);
            procedures.push(proc);
        }

        ProcedureChain {
            proc_registry: registry,
            procedures,
        }
    }

    pub fn run(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()> {
        for procedure in self.procedures.iter() {
            procedure.run(graphs)?;
        }
        Ok(())
    }
}
