use crate::graph::undirected::simple_graph::SimpleGraph;
use crate::procedure::basic_impl::basic_procedure::BasicProcedure;
use crate::procedure::configuration::Configuration;
use crate::procedure::procedure_chain::ProcedureChain;
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

fn main() {
    let args = Cli::from_args();

    match args.command.as_ref() {
        "run" => {
            let config_str =
                std::fs::read_to_string(&args.config_file_path).expect("could not read file");
            let config = parse_yaml_config(&config_str);

            let chain = ProcedureChain::<BasicProcedure>::from_procedures_config(config.procedures);
            let mut graphs: Vec<SimpleGraph> = vec![];
            // let mut graphs_with_properties: Vec<(SimpleGraph)> = vec![];

            match chain.run(&mut graphs) {
                Err(error) => {
                    eprintln!("Error: {}", error);
                }
                Ok(()) => {}
            }
        }
        _ => {
            println!("Unknown command");
        }
    }
}
