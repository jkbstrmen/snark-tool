use std::fmt::Debug;
use crate::procedure::procedure::Procedure;
use crate::procedure::graph::Graph;

pub struct ProcedureChain<Procedure> {
    procedures: Vec<Procedure>,
}

impl<P> ProcedureChain<P> where P: Procedure {
    // add_procedure
    pub(crate) fn new() -> Self {
        ProcedureChain{ procedures: vec![] }
    }

    pub fn add_procedure(&mut self, procedure: P) {
        self.procedures.push(procedure);
    }

    // run
    pub fn run<G>(&self, graph: &mut G) where G: Debug + Graph {
        // println!("graph: {:?}", graph);
        println!("Run chain");

        for procedure in self.procedures.iter(){
            procedure.run(graph);
        }

    }
}
