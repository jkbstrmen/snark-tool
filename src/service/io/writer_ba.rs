use std::fs::OpenOptions;
use std::io::{LineWriter, Read, Write};

use std::path::Path;

use petgraph::Undirected;

use petgraph::stable_graph::StableGraph;
use petgraph::visit::EdgeRef;

use crate::service::io::reader_ba::get_graphs_count_with_preface;

// TODO - handle errors
pub fn write_graph_ba(
    graph: StableGraph<u8, u16, Undirected, u8>,
    index: u32,
    mut buffer: impl Write,
) {
    writeln!(buffer);
    writeln!(buffer, "{}", index);
    for node_index in graph.node_indices() {
        for edge_ref in graph.edges(node_index) {
            write!(buffer, "{} ", edge_ref.target().index());
        }
        writeln!(buffer);
    }
}

// TODO - handle errors
fn update_graphs_count(path: impl AsRef<Path>, new_count: usize, preface: String) {
    let file_result = OpenOptions::new().write(true).open(path);
    let mut writer = LineWriter::new(file_result.unwrap());
    writer.write_all(preface.as_bytes());
    let count_str = format!("{}", new_count);
    writer.write_all(count_str.as_bytes());
}

// TODO - handle errors
pub fn append_graph_ba_to_file(
    graph: StableGraph<u8, u16, Undirected, u8>,
    path: impl AsRef<Path>,
)
/*-> Result<>*/
{
    let file_result = OpenOptions::new().read(true).open(&path);

    let count_preface_result = get_graphs_count_with_preface(&file_result.unwrap());
    println!("{:?}", count_preface_result);
    let mut count = 0;
    let mut new_count = 0;
    if count_preface_result.is_ok() {
        let count_preface = count_preface_result.unwrap();
        count = count_preface.0;
        let preface = count_preface.1;
        new_count = count + 1;
        update_graphs_count(&path, new_count, preface);
    }

    let file_result = OpenOptions::new().append(true).open(&path);
    let mut file = file_result.unwrap();

    write_graph_ba(graph, (new_count) as u32, &mut file);
}
