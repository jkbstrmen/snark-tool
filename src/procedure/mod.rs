mod procedure;
mod procedure_chain;
mod graph;
pub mod configuration;

use crate::procedure::procedure_chain::ProcedureChain;
use crate::procedure::procedure::BasicProcedure;
use crate::procedure::graph::SimpleGraph;
use yaml_rust::Yaml;
use yaml_rust::yaml::Hash;

pub fn create_procedure_chain(yaml_procedures: &Hash){

    // println!("procedures: {:?}", yaml_procedures);


    // for each procedure add procedure to chain
    let mut chain: ProcedureChain<BasicProcedure> = ProcedureChain::new();

    for proc in yaml_procedures.iter() {
        let name = proc.0.as_str();
        // println!("procedure name: {:?}", name.expect("Oooops"));
        // println!("  procedure body: {:?}", proc.1);

        let proc = BasicProcedure::new(name.expect("Oooops"));
        chain.add_procedure(proc);
    }


    let mut graph = SimpleGraph{ graph: "Hello".to_string() };
    // let mut graphs: Vec<SimpleGraph> = Vec::new();
    let mut graphs: Vec<SimpleGraph> = vec![graph];
    chain.run(&mut graphs);

}

pub fn procedures_playground() {
    let mut chain: ProcedureChain<BasicProcedure> = ProcedureChain::new();

    let proc = BasicProcedure::new("read");
    let proc2 = BasicProcedure::new(String::from("write"));

    chain.add_procedure(proc);
    chain.add_procedure(proc2);

    // let graph = String::from("Hello");
    let mut graph = SimpleGraph{ graph: "Hello".to_string() };
    // chain.run(&mut graph);

}
