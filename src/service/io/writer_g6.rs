use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::Undirected;
use petgraph::visit::EdgeRef;

use crate::service::io::reader_g6::{Position, BIAS};
use std::io::Write;

type Graph = StableGraph<u8, u16, Undirected, u8>;

// TODO - handle io errors
pub fn write_graph(graph: Graph, buffer: &mut impl Write) {
    let graph_string = to_g6_string(graph);
    writeln!(buffer, "{}", graph_string);
}

fn to_g6_string(graph: Graph) -> String {
    let mut graph_string = String::new();
    graph_string.push_str(to_g6_size(graph.node_count()).as_ref());
    let size = graph.node_count();
    let mut position = Position { row: 0, column: 1 };

    let mut binary = String::new();
    loop {
        if has_edge(&graph, position.row, position.column) {
            binary.push('1');
        } else {
            binary.push('0');
        }

        if binary.len() == 6 {
            let intval = u8::from_str_radix(binary.as_str(), 2).unwrap();
            graph_string.push((intval + BIAS) as char);
            binary = String::new();
        }
        position.increment();
        if position.row > size || position.column > size {
            break;
        }
    }
    trim_ending_zeros(&mut graph_string);
    graph_string
}

pub fn to_g6_size(size: usize) -> String {
    let mut size_string = String::new();

    if size < 63 {
        size_string.push((size as u8 + BIAS) as char);
        return size_string;
    }
    if size > 62 && size <= 258047 {
        size_string.push_str(to_medium_size_string(size).as_ref());
        return size_string;
    }
    if size > 258047 && size <= 68719476735 {
        size_string.push_str(to_big_size_string(size).as_ref());
        return size_string;
    }
    size_string
}

// move to Graph fn
fn has_edge(graph: &Graph, source: usize, target: usize) -> bool {
    let edges = graph.edges(NodeIndex::new(source));
    for edge in edges {
        if edge.target().index().eq(&target) {
            return true;
        }
    }
    false
}

fn trim_ending_zeros(graph_string: &mut String) {
    loop {
        let char = graph_string.pop();
        if char.is_some() && char.unwrap() != '?' {
            graph_string.push(char.unwrap());
            return;
        }
        if graph_string.is_empty() {
            return;
        }
    }
}

fn to_medium_size_string(mut size: usize) -> String {
    let mut size_string = String::new();
    size_string.push((shift(&mut size) + BIAS) as char);
    size_string.push((shift(&mut size) + BIAS) as char);
    size_string.push((shift(&mut size) + BIAS) as char);
    size_string.push('~');
    revert(size_string)
}

fn to_big_size_string(mut size: usize) -> String {
    let mut size_string = String::new();
    size_string.push((shift(&mut size) + BIAS) as char);
    size_string.push((shift(&mut size) + BIAS) as char);
    size_string.push((shift(&mut size) + BIAS) as char);
    size_string.push((shift(&mut size) + BIAS) as char);
    size_string.push((shift(&mut size) + BIAS) as char);
    size_string.push((shift(&mut size) + BIAS) as char);
    size_string.push('~');
    size_string.push('~');
    revert(size_string)
}

fn revert(mut orig: String) -> String {
    let mut reverted = String::new();
    let mut char = orig.pop();
    while char.is_some() {
        reverted.push(char.unwrap());
        char = orig.pop();
    }
    reverted
}

fn shift(num: &mut usize) -> u8 {
    let orig = num.clone();
    let mut shifted = *num >> 6;
    *num = shifted;
    shifted <<= 6;
    (orig - shifted) as u8
}
