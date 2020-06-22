use crate::graph::undirected::simple_graph::SimpleGraph;
use crate::procedure::configuration::Configuration;
use crate::procedure::procedure_chain::ProcedureChain;
use crate::procedure::procedure_registry::ProcedureRegistry;
use crate::service::chromatic_properties::resistance::Resistance;
use crate::service::colour::bfs::BFSColourizer;
use crate::service::colour::colouriser::Colourizer;
use crate::service::colour::sat::SATColourizer;
use crate::service::io::reader_g6::G6Reader;
use std::collections::HashMap;
use std::time::Instant;
use structopt::StructOpt;

mod error;
mod graph;
mod procedure;
mod service;
mod test;

/// Simple tool for snark analysis. For more information visit https://github.com/jstrmen/snark-tool
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

type BasicProperties = HashMap<String, String>;

fn main() {
    let args = Cli::from_args();

    match args.command.as_ref() {
        "run" => {
            let begin = Instant::now();

            let config_str =
                std::fs::read_to_string(&args.config_file_path).expect("could not read file");
            let config = parse_yaml_config(&config_str);

            let mut registry = ProcedureRegistry::new_basic();
            // add builder of own procedure impl to registry as below
            // registry.insert("read".to_string(), ReadProcedureBuilder{});

            let chain = ProcedureChain::from_procedures_config(registry, config.procedures);
            let mut graphs_with_properties: Vec<(SimpleGraph, BasicProperties)> = vec![];
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

// use crate::service::colour::sat;
// use crate::service::io::reader::Reader;
// use crate::service::io::reader_ba::BaReader;
// use std::fs::OpenOptions;
// use std::time::Instant;
//
// fn main() {
//     let begin = Instant::now();
//
//     let file = OpenOptions::new()
//         .read(true)
//         .open("../../resources/graphs/PSC6XJ5.118")
//         .unwrap();
//     let mut reader = BaReader::<SimpleGraph>::new(&file);
//     let graph = reader.next().unwrap();
//
//     let solution = sat::is_colorable(&graph.unwrap());
//     println!("solution: {}", solution);
//     println!("elapsed: {}ms", begin.elapsed().as_millis());
// }
