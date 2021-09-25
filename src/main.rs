use std::time::Instant;

use structopt::StructOpt;

use crate::graph::undirected::simple_graph::graph::SimpleGraph;
use crate::procedure::configuration::Configuration;
use crate::procedure::procedure::GraphProperties;
use crate::procedure::procedure_chain::ProcedureChain;
use crate::procedure::procedure_registry::ProcedureRegistry;

mod graph;
mod procedure;
mod service;
mod tests;

/// Simple tool for snark analysis. For more information visit https://github.com/jkbstrmen/snark-tool
#[derive(StructOpt)]
struct Cli {
    /// Command - e.g. 'run'
    command: String,
    /// The path to the configuration file - e.g. 'snark-tool.yml'
    #[structopt(parse(from_os_str))]
    config_file_path: std::path::PathBuf,
}

fn parse_yaml_config(source: &String) -> Configuration {
    let config = match Configuration::from_yaml_string(source) {
        Ok(configuration) => configuration,
        Err(error) => panic!("Configuration parse error: {}", error),
    };
    config
}

fn main() {
    let args = Cli::from_args();

    match args.command.as_ref() {
        "run" => {
            let begin = Instant::now();

            let config_str =
                std::fs::read_to_string(&args.config_file_path).expect("could not read file");
            let config = parse_yaml_config(&config_str);

            let registry = ProcedureRegistry::new_basic();
            // add builder of own procedure impl to registry as shown below
            // registry.insert("read".to_string(), ReadProcedureBuilder{});

            let chain = ProcedureChain::from_procedures_config(registry, config.procedures);
            if chain.is_err() {
                eprintln!("Error: {}", chain.err().unwrap());
                return;
            }

            let chain = chain.unwrap();
            let mut graphs_with_properties: Vec<(SimpleGraph, GraphProperties)> = vec![];
            match chain.run(&mut graphs_with_properties) {
                Err(error) => {
                    eprintln!("Error: {}", error);
                }
                Ok(()) => {}
            }
            println!("elapsed: {}ms", begin.elapsed().as_millis());
        }
        _ => {
            println!("Unknown command");
        }
    }
}
