use std::collections::HashMap;

use crate::graph::undirected::simple_graph::SimpleGraph;
use crate::procedure::configuration::ProcedureConfig;
use crate::procedure::procedure::BasicProcedure;
use crate::procedure::procedure_chain::ProcedureChain;

pub mod configuration;
mod procedure;
mod procedure_chain;

// TODO - parametrize function with generic Procedure type
// move to proc chain?

pub fn create_procedure_chain(
    mut proc_configs: Vec<ProcedureConfig>,
) -> ProcedureChain<BasicProcedure> {
    let mut procedures: Vec<BasicProcedure> = vec![];
    while !proc_configs.is_empty() {
        if let Some(proc_config) = proc_configs.pop() {
            let config = match proc_config.config {
                Some(map) => map,
                None => HashMap::default(),
            };
            let proc = BasicProcedure::new_with_config(proc_config.proc_type, config);
            procedures.push(proc);
        };
    }
    procedures.reverse();
    let chain: ProcedureChain<BasicProcedure> = ProcedureChain::from_procedures_vector(procedures);

    chain
}

pub fn procedures_playground(proc_configs: Vec<ProcedureConfig>) {
    let chain = create_procedure_chain(proc_configs);

    // let graph = SimpleGraph {
    //     graph: "Hello".to_string(),
    // };
    // // let mut graphs: Vec<SimpleGraph> = Vec::new();
    // let mut graphs: Vec<SimpleGraph> = vec![graph];
    // chain.run(&mut graphs);
}
