use crate::error::Error;
use crate::graph::traits::graph::Graph;
use crate::procedure::configuration::ProcedureConfig;
use crate::procedure::procedure::Procedure;
use std::collections::HashMap;
use std::fmt::Debug;
use std::result;

type Result<T> = result::Result<T, Error>;

pub struct ProcedureChain<Procedure> {
    procedures: Vec<Procedure>,
}

impl<P> ProcedureChain<P>
where
    P: Procedure,
{
    // pub fn new() -> Self {
    //     ProcedureChain { procedures: vec![] }
    // }

    pub fn from_procedures_vector(procedures: Vec<P>) -> Self {
        ProcedureChain { procedures }
    }

    // pub fn add_procedure(&mut self, procedure: P) {
    //     self.procedures.push(procedure);
    // }

    pub fn from_procedures_config(mut proc_configs: Vec<ProcedureConfig>) -> Self {
        let mut procedures: Vec<P> = vec![];
        while !proc_configs.is_empty() {
            if let Some(proc_config) = proc_configs.pop() {
                let config = match proc_config.config {
                    Some(map) => map,
                    None => HashMap::default(),
                };
                let proc = P::new_with_config(proc_config.proc_type, config);
                procedures.push(proc);
            };
        }
        procedures.reverse();
        let chain: ProcedureChain<P> = ProcedureChain::from_procedures_vector(procedures);

        chain
    }

    pub fn run<G /*, Prop*/>(&self, graphs: &mut Vec<G /*(G, Prop)*/>) -> Result<()>
    where
        G: Debug + Graph,
    {
        for procedure in self.procedures.iter() {
            procedure.run(graphs)?;
        }
        Ok(())
    }
}
