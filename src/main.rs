use structopt::StructOpt;
// use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

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

    parse_config(&content);
}

fn parse_config(source: &String) {
    let docs = YamlLoader::load_from_str(source).unwrap();

    let doc = &docs[0];

    // println!("{:?}", doc);

    let is_null = doc["version"].is_null();
    println!("is null: {}", is_null);

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

    let is_null = doc["procedures"].is_null();
    println!("procedures is null: {}", is_null);
    let procedures = match doc["procedures"].as_hash() {
        Some(vec) => {
            vec
        }
        None => {
            panic!("crash and burn");
        }
    };
    println!("procedures: {:?}", procedures)

    // let version = doc["version"].into_string();
    // let version = doc["version"].as_i64().unwrap();
    // println!("version: {:?}", version);

    // println!("{:?}", docs[0]);
}

fn main() {
    read_args();
    println!("Hello, world!");
}
