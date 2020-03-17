use petgraph::stable_graph::StableGraph;
use petgraph::Undirected;

use crate::graph::pet_graph_utils::get_edges_vec;
use std::io::Write;

use crate::service::io::reader_g6::BIAS;
use crate::service::io::writer_g6::to_g6_size;
use std::cmp;
use std::cmp::Ordering;
use petgraph::graph::NodeIndex;
// use log::{info, trace, warn, debug};

type Graph = StableGraph<u8, u16, Undirected, u8>;

pub const ENCODING_SIZE: u8 = 6;

pub fn write_graph(graph: &Graph, buffer: &mut impl Write) {
    let graph_string = to_s6_string(graph);
    writeln!(buffer, "{}", graph_string);
}

pub fn to_s6_string(graph: &Graph) -> String {
    let mut edges = get_edges_vec(graph);

    edges.sort_by(|a, b| edge_max_min_compare(a, b));

    let mut encoded = encode_edges(graph.node_count(), &edges);
    // complete encoding of six
    complete_to_multiple_of_six(&mut encoded, graph);

    let edges_s6_chars = to_s6_chars(encoded);
    let mut graph_s6 = String::from(":");

    graph_s6.push_str(to_g6_size(graph.node_count()).as_str());
    graph_s6.push_str(edges_s6_chars.as_str());
    graph_s6
}

fn to_s6_chars(mut edges_encoding: Vec<bool>) -> String {
    if edges_encoding.len() % 6 != 0 {
        //
    }
    let mut s6_chars = String::new();
    let mut begin = 0;
    let mut end = 6;
    while begin < edges_encoding.len() {
        let mut char: u8 = 0;
        for i in begin..end {
            let bit = edges_encoding.get(i).unwrap().clone();
            char = (char << 1) | bit as u8;
        }
        s6_chars.push((char + BIAS) as char);
        begin += 6;
        end += 6;
    }
    s6_chars
}

fn complete_to_multiple_of_six(encoding: &mut Vec<bool>, graph: &Graph) {
    let rem = encoding.len() % 6;
    let completion = 6 - rem;
    check_completion_edge_case(encoding, graph);
    let mut completion = vec![true; completion];
    encoding.append(&mut completion);
}

fn check_completion_edge_case(encoding: &mut Vec<bool>, graph: &Graph){
    let n = graph.node_count();
    if n == 2 || n == 4 || n == 8 || n == 16 {
        let mut n1_edges = graph.edges(NodeIndex::new(n-1));
        let mut n2_edges = graph.edges(NodeIndex::new(n-2));
        if n2_edges.next().is_some() && n1_edges.next().is_none() {
            let edge_encoding_size = (n as f64).log(2 as f64).ceil() as u8;
            let rem = encoding.len() % 6;
            let complement = 6 - rem;
            if complement > edge_encoding_size as usize {
                encoding.push(false);
            }

        }
    }
}

pub fn encode_edges(size: usize, edges: &Vec<(usize, usize)>) -> Vec<bool> {
    let edge_encoding_size = (size as f64).log(2 as f64).ceil() as u8;
    let mut v: usize = 0;
    let mut vec: Vec<bool> = Vec::new();
    for edge in edges {
        if edge.1 > (v + 1) {
            // shift v
            vec.push(false);
            vec.append(&mut bitvec_from_u64(edge.1 as u64, edge_encoding_size));
            v = edge.1;
        }
        if edge.1 == v + 1 {
            vec.push(true);
            v = v + 1;
        } else {
            vec.push(false);
        }
        vec.append(&mut bitvec_from_u64(edge.0 as u64, edge_encoding_size));
    }
    vec
}

fn bitvec_to_u64(bitvec: &Vec<bool>) -> u64 {
    if bitvec.len() <= 8 {
        let mut result: u8 = 0;
        for bit in bitvec {
            println!("{}", result);
            result = (result << 1) | bit.clone() as u8;
        }
        return result as u64;
    }
    0
}

pub fn bitvec_from_u64(mut num: u64, bits_count: u8) -> Vec<bool> {
    let mut vec = vec![];
    loop {
        let rem = (num % 2) != 0;
        vec.push(rem);
        num >>= 1;
        if num == 0 {
            break;
        }
    }
    while vec.len() < bits_count as usize {
        vec.push(false);
    }
    vec.reverse();
    vec
}

fn encode_edge() {}

fn edge_max_min_compare(first: &(usize, usize), second: &(usize, usize)) -> Ordering {
    let max_first = cmp::max(first.0, first.1);
    let max_second = cmp::max(second.0, second.1);
    let compare_max = max_first.cmp(&max_second);
    if Ordering::Equal.eq(&compare_max) {
        let min_first = cmp::min(first.0, first.1);
        let min_second = cmp::min(second.0, second.1);
        return min_first.cmp(&min_second);
    }
    compare_max
}
