use structopt::StructOpt;
use yaml_rust::{YamlLoader, Yaml};
use crate::procedure::configuration::Configuration;
use serde_yaml::Error;


mod procedure;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// Command
    command: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn read_args() {
    let args = Cli::from_args();
    println!("command: {}", args.command);
    println!("path to file: {:?}", args.path);

    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    // println!("File content: {}", content);

    parse_yaml_config(&content);
}

fn parse_yaml_config(source: &String) -> Result<(), serde_yaml::Error> {

    // TODO - handle error somehow else
    let config: Configuration = serde_yaml::from_str(&source)?;



    let docs = YamlLoader::load_from_str(source).unwrap();
    let doc = &docs[0];

    // let is_null = doc["version"].is_null();
    // println!("doc is null: {}", is_null);

    let is_null = doc["procedures"].is_null();
    println!("procedures is null: {}", is_null);
    let procedures = match doc["procedures"].as_hash() {
        Some(hash) => hash,
        None => {
            panic!("crash and burn");
        }
    };
    // println!("procedures: {:?}", procedures);

    procedure::create_procedure_chain(procedures);

    Ok(())
}

fn get_version(doc: &Yaml){
    let version = match doc["version"].as_f64() {
        Some(num) => {
            println!("Someeeee: {}", num);
            num
        }
        None => {
            println!("Noneeeee");
            panic!("crash and burn");
            //String::from("None")
        }
    };
    println!("version is: {}", version);
}

fn main() {
    read_args();
    // procedure::procedures_playground();

    println!("Hello, world!");
}
