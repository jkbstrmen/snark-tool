use snark_tool::graph::undirected::simple_graph::graph::SimpleGraph;
use snark_tool::service::io::reader::GraphFileReader;
use snark_tool::service::io::reader_g6::G6Reader;
use std::fs;

fn main() {
    let input_file_path = "resources/test/Generated_graphs.30.05.sn.cyc4.100.g6";
    let input_file = fs::OpenOptions::new()
        .read(true)
        .open(&input_file_path)
        .unwrap();
    let mut reader = G6Reader::<SimpleGraph>::new(&input_file);

    while let Some(graph) = reader.next() {
        println!("{:?}", graph.unwrap());
    }
}
