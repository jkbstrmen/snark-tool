use crate::graph::graph::{Graph, Edge, SimpleGraph};
use std::collections::HashMap;
use std::fmt::Debug;
use crate::error::Error;
use std::result;
use std::str::FromStr;
use std::fs::OpenOptions;
use crate::service::io::reader_g6::G6Reader;

type Config = HashMap<String, String>;
type Result<T> = result::Result<T, Error>;

pub trait Procedure {
    fn run<G>(&self, graphs: &mut Vec<G>) -> Result<()>
    where
        G: Debug + Graph;
}

pub struct BasicProcedure {
    proc_type: String,
    config: Config,
}

impl BasicProcedure {
    pub fn new(proc_type: impl AsRef<str>) -> Self {
        BasicProcedure {
            proc_type: String::from(proc_type.as_ref()),
            config: HashMap::default(),
        }
    }

    pub fn new_with_config(proc_type: impl AsRef<str>, config: Config) -> Self {
        BasicProcedure {
            proc_type: String::from(proc_type.as_ref()),
            config,
        }
    }
}

impl Procedure for BasicProcedure {
    fn run<G>(&self, graphs: &mut Vec<G>) -> Result<()>
    where
        G: Debug + Graph,
    {
        match self.proc_type.as_str() {
            "read" => {
                self.read_graph(graphs)?;
            }
            "write" => {
                self.write_graph(graphs)?;
            }
            "colour" => {
                self.colour_graph(graphs)?;
            }
            _ => {
                self.handle_unknown_type();
            }
        };
        Ok(())
    }
}

impl BasicProcedure {
    fn read_graph<G>(&self, graphs: &mut Vec<G>) -> Result<()>
    where
        G: Debug + Graph,
    {

        // TODO - read graphs from file -> add to graphs

        println!(
            "Running procedure: {} on graph: {:?}",
            self.proc_type, graphs
        );

        // println!("Read graph config: {:?}", self.config);
        let graph_path = self.config.get("file").expect("file path not specified");
        // println!("graph file path: {}", graph_path);
        // let content = std::fs::read_to_string(graph_path).expect("could not read file");
        // println!("Graph file content: {}", content);

        let graphs_count_opt = self.config.get("number-of-graphs");
        // if not specified - read all graphs from file
        // handle unwrap
        let graphs_count = u64::from_str(graphs_count_opt.unwrap().clone().as_str()).unwrap();
        println!("Graphs count to read: {}", graphs_count);

        let file_result = OpenOptions::new().read(true).open(graph_path);

        let reader = G6Reader::<SimpleGraph>::new();
        let graphs = G6Reader::<SimpleGraph>::read_by_lines(&file_result.unwrap(), graphs_count)?;

        // mut buffer: io::Lines<io::BufReader<&File>>



        // number-of-graphs

        // let graph = SimpleGraph{ graph: content };
        // let graph = G::from_str(content.as_str());
        // graphs.push(graph);

        Ok(())
    }

    fn write_graph<Graph>(&self, graphs: &mut Vec<Graph>) -> Result<()>
    where
        Graph: Debug,
    {
        println!(
            "Running procedure: {} on graphs: {:?}",
            self.proc_type, graphs
        );
        Ok(())
    }

    fn colour_graph<G>(&self, graphs: &mut Vec<G>) -> Result<()>
    where
        G: Debug + Graph,
    {
        println!(
            "Running procedure: {} on graphs: {:?}",
            self.proc_type, graphs
        );
        Ok(())
    }

    fn handle_unknown_type(&self) {
        println!("Unknown procedure type: {}", self.proc_type);
    }
}
