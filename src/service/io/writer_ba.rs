use std::fs::File;
use std::io::{self, BufRead, Write, Read};

use petgraph::{Graph, Undirected};
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;

use crate::service::io::reader_ba::get_graphs_count;

// append
pub fn append_graph_ba_to_file(graph: StableGraph<u8, u16, Undirected, u8>, file: &mut File){

    // TODO - handle read write errors

    // resolve current graph count in file

    // let linesReader = io::BufReader::new(file).lines();
    // let count = get_graphs_count(linesReader);

    // increase this count

    // append graph description to the end of file

    let mut s = String::from("");
    file.read_to_string(&mut s).unwrap();
    println!("{}", s);

    file.write("\n hello \n".as_bytes());

    // let graph_str= format!("{:?}", graph);
    // file.write(graph_str.as_bytes());


}