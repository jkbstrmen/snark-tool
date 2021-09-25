use snark_tool::graph::undirected::simple_graph::graph::SimpleGraph;
use snark_tool::service::colour::colouriser::Colouriser;
use snark_tool::service::colour::recursive::dfs_improved::DFSColourizer;
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

    while let Some(graph_result) = reader.next() {
        let graph = graph_result.unwrap();
        let colourable = DFSColourizer::is_colorable(&graph);
        println!("COLOURABLE: {}", colourable);
    }
}
