use crate::graph::graph::GraphConstructor;
use crate::graph::undirected::UndirectedGraph;
use crate::procedure::configuration::ProcedureConfig;
use crate::procedure::procedure::{GraphProperties, Procedure, Result};
use crate::procedure::procedure_registry::ProcedureRegistry;

pub struct ProcedureChain<G: UndirectedGraph> {
    _proc_registry: ProcedureRegistry<G>,
    procedures: Vec<Box<dyn Procedure<G>>>,
}

impl<G: UndirectedGraph + GraphConstructor + Clone + 'static> ProcedureChain<G> {
    pub fn from_procedures_config(
        registry: ProcedureRegistry<G>,
        configurations: Vec<ProcedureConfig>,
    ) -> Result<Self> {
        let mut procedures = vec![];
        for configuration in configurations {
            let proc = registry.create_procedure(configuration)?;
            procedures.push(proc);
        }

        Ok(ProcedureChain {
            _proc_registry: registry,
            procedures,
        })
    }

    // TODO add impl to create chain from procedure objects

    pub fn run(&self, graphs: &mut Vec<(G, GraphProperties)>) -> Result<()> {
        for procedure in self.procedures.iter() {
            procedure.run(graphs)?;
        }
        Ok(())
    }
}
