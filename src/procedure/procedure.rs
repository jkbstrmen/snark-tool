use crate::graph::graph::{Graph, SimpleGraph};
use std::collections::HashMap;
use std::fmt::Debug;

type Config = HashMap<String, String>;

pub trait Procedure {
    fn run<G>(&self, graphs: &mut Vec<G>)
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
    fn run<G>(&self, graphs: &mut Vec<G>)
    where
        G: Debug + Graph,
    {
        match self.proc_type.as_str() {
            "read" => {
                self.read_graph(graphs);
            }
            "write" => {
                self.write_graph(graphs);
            }
            "colour" => {
                self.colour_graph(graphs);
            }
            _ => {
                self.handle_unknown_type();
            }
        };
    }
}

impl BasicProcedure {
    fn read_graph<G>(&self, graphs: &mut Vec<G>)
    where
        G: Debug + Graph,
    {
        let s = String::from("ola");
        // s.push_str("read");

        println!(
            "Running procedure: {} on graph: {:?}",
            self.proc_type, graphs
        );

        println!("Read graph config: {:?}", self.config);
        let graph_path = self.config.get("file").expect("file path not specified");
        println!("graph file path: {}", graph_path);
        let content = std::fs::read_to_string(graph_path).expect("could not read file");
        println!("Graph file content: {}", content);

        // let graph = SimpleGraph{ graph: content };

        let graph = G::from_str(content.as_str());
        graphs.push(graph);
        // graph.add_edge(" read");

        // read graphs from file or string from config and add to vector of graphs (specified number)
    }

    fn write_graph<Graph>(&self, graphs: &mut Vec<Graph>)
    where
        Graph: Debug,
    {
        println!(
            "Running procedure: {} on graph: {:?}",
            self.proc_type, graphs
        );
    }

    fn colour_graph<G>(&self, graphs: &mut Vec<G>)
    where
        G: Debug + Graph,
    {
        println!(
            "Running procedure: {} on graph: {:?}",
            self.proc_type, graphs
        );
    }

    fn handle_unknown_type(&self) {
        println!("Unknown procedure type: {}", self.proc_type);
    }
}
