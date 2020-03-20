use crate::error::Error;
use crate::graph::simple_graph::SimpleGraph;
use crate::procedure::configuration::Configuration;
use structopt::StructOpt;

mod error;
mod graph;
mod procedure;
mod service;
mod test;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// Command
    command: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn read_config() -> String {
    let args = Cli::from_args();
    println!("command: {}", args.command);
    println!("path to file: {:?}", args.path);

    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    content
}

fn parse_yaml_config(source: &String) -> Configuration {
    let config = match Configuration::from_yaml_string(source) {
        Ok(configuration) => configuration,
        Err(error) => panic!("Configuration parse error: {}", error),
    };
    config
}

fn main() {
    let config_str = read_config();
    let config = parse_yaml_config(&config_str);

    let chain = procedure::create_procedure_chain(config.procedures);
    let mut graphs: Vec<SimpleGraph> = vec![];

    match chain.run(&mut graphs) {
        Err(error) => {
            eprintln!("Error: {}", error);
        }
        Ok(()) => {}
    }

    // procedure::procedures_playground(config.procedures);

    println!("Hello, world!");
}
