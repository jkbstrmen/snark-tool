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

    pub fn from_procedures_and_registry(
        procedures: Vec<Box<dyn Procedure<G>>>,
        registry: ProcedureRegistry<G>,
    ) -> Result<Self> {
        Ok(Self {
            _proc_registry: registry,
            procedures,
        })
    }

    pub fn from_procedures(procedures: Vec<Box<dyn Procedure<G>>>) -> Result<Self> {
        let registry = ProcedureRegistry::new_basic();
        Self::from_procedures_and_registry(procedures, registry)
    }

    pub fn run(&self, graphs: &mut Vec<(G, GraphProperties)>) -> Result<()> {
        for procedure in self.procedures.iter() {
            procedure.run(graphs)?;
        }
        Ok(())
    }
}
