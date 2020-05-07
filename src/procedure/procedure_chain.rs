use crate::error::Error;
use crate::graph::graph::{Graph, GraphConstructor};
use crate::procedure::configuration::ProcedureConfig;
use crate::procedure::procedure::Procedure;
use std::collections::HashMap;
use std::fmt::Debug;
use std::{marker, result};

type Result<T> = result::Result<T, Error>;

pub struct ProcedureChain<Procedure, Prop> {
    procedures: Vec<Procedure>,

    _ph: marker::PhantomData<Prop>,
}

impl<P, Prop> ProcedureChain<P, Prop>
where
    P: Procedure<Prop>,
{
    pub fn from_procedures_vector(procedures: Vec<P>) -> Self {
        ProcedureChain {
            procedures,
            _ph: marker::PhantomData,
        }
    }

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
        let chain: ProcedureChain<P, Prop> = ProcedureChain::from_procedures_vector(procedures);

        chain
    }

    pub fn run<G>(&self, graphs: &mut Vec<(G, Prop)>) -> Result<()>
    where
        G: Debug + Graph + GraphConstructor,
    {
        for procedure in self.procedures.iter() {
            procedure.run(graphs)?;
        }
        Ok(())
    }
}
