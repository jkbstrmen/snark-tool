use std::fmt::Debug;
use crate::procedure::graph::Graph;
use std::collections::HashMap;

// type Config = LinkedHashMap<String, String>;
type Config = HashMap<String, String>;

pub trait Procedure{
    // fn run<G>(&self, graph: &mut G) where G: Debug + Graph;
    fn run<G>(&self, graphs: &mut Vec<G>) where G: Debug + Graph;
}

// struct Procedure {
//     proc_type: String,
// }

pub struct BasicProcedure {
    proc_type: String,
    config: Config
}

impl BasicProcedure{

    pub fn new(proc_type: impl AsRef<str>) -> Self {
        BasicProcedure{ proc_type: String::from(proc_type.as_ref()), config: HashMap::default() }
    }
    // pub fn new(proc_type: &String) -> Self {
    //     BasicProcedure{ proc_type: proc_type.to_string() }
    // }
}

impl Procedure for BasicProcedure {
    fn run<G>(&self, graphs: &mut Vec<G>) where G: Debug + Graph {

        match self.proc_type.as_str() {
            "read" => {
                // println!("Reading");
                self.read_graph(graphs);
            }
            "write" => {
                // println!("Writing");
                self.write_graph(graphs);
            }
            _ => {
                self.handle_unknown_type();
            }
        };

        // println!("Running procedure: {} on graph: {:?}", self.proc_type, graph);
    }
}

impl BasicProcedure{
    fn read_graph<G>(&self, graphs: &mut Vec<G>) where G: Debug + Graph {

        let s = String::from("ola");
        // s.push_str("read");

        println!("Running procedure: {} on graph: {:?}", self.proc_type, graphs);
        // graph.add_edge(" read");

        // read graphs from file or string from config and add to vector of graphs (specified number)


    }

    fn write_graph<Graph>(&self, graphs: &mut Vec<Graph>) where Graph: Debug {

        println!("Running procedure: {} on graph: {:?}", self.proc_type, graphs);

    }

    fn handle_unknown_type(&self){
        println!("Unknown procedure type: {}", self.proc_type);
    }
}

