use crate::graph::graph::Graph;
use crate::procedure::procedure::Procedure;
use std::fmt::Debug;

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

    pub fn run<G>(&self, graphs: &mut Vec<G>)
    where
        G: Debug + Graph,
    {
        for procedure in self.procedures.iter() {
            procedure.run(graphs);
        }
    }
}
