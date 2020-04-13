use std::io::Write;

use crate::graph::traits::edge::Edge;
use crate::graph::traits::graph::Graph;
use crate::service::io::error::WriteError;
use crate::service::io::reader_g6::BIAS;
use crate::service::io::writer_g6::to_g6_size;
use std::cmp::Ordering;
use std::fs::OpenOptions;
use std::iter::FromIterator;
use std::{cmp, marker, path, result};

// pub const S6_CONSTANT: u8 = 6;
type Result<T> = result::Result<T, WriteError>;

pub struct S6Writer<G> {
    _ph: marker::PhantomData<G>,
}

impl<G> S6Writer<G>
where
    G: Graph,
{
    pub fn write_graphs_to_file<P>(
        graphs: &Vec<(G, P)>,
        path: impl AsRef<path::Path>,
    ) -> Result<()> {
        let file_result = OpenOptions::new().create(true).append(true).open(&path);
        if let Err(err) = &file_result {
            return Err(WriteError {
                message: format!("open or create file error: {}", err),
            });
        }
        let mut file = file_result.unwrap();
        for graph in graphs {
            S6Writer::write_graph(&graph.0, &mut file)?;
        }
        Ok(())
    }

    pub fn write_graph(graph: &G, buffer: &mut impl Write) -> Result<()> {
        let graph_string = S6Writer::graph_to_s6_string(graph);
        writeln!(buffer, "{}", graph_string)?;
        Ok(())
    }

    pub fn graph_to_s6_string(graph: &G) -> String {
        let mut edges = Vec::from_iter(graph.edges());
        let size = graph.size();

        edges.sort_by(|a, b| edge_max_min_compare(a, b));

        let mut encoded = encode_edges(size, &edges);
        // complete encoding of six
        S6Writer::complete_to_multiple_of_six(&mut encoded, graph);

        let edges_s6_chars = to_s6_chars(encoded);
        let mut graph_s6 = String::from(":");

        graph_s6.push_str(to_g6_size(size).as_str());
        graph_s6.push_str(edges_s6_chars.as_str());
        graph_s6
    }

    fn complete_to_multiple_of_six(encoding: &mut Vec<bool>, graph: &G) {
        let rem = encoding.len() % 6;
        let completion = 6 - rem;
        S6Writer::check_completion_edge_case(encoding, graph);
        let mut completion = vec![true; completion];
        encoding.append(&mut completion);
    }

    fn check_completion_edge_case(encoding: &mut Vec<bool>, graph: &G) {
        let n = graph.size();
        if n == 2 || n == 4 || n == 8 || n == 16 {
            let mut n1_edges = graph.edges_of_vertex(n - 1);
            let mut n2_edges = graph.edges_of_vertex(n - 2);
            if n2_edges.next().is_some() && n1_edges.next().is_none() {
                let edge_encoding_size = edge_encoding_size(n);
                let rem = encoding.len() % 6;
                let complement = 6 - rem;
                if complement > edge_encoding_size as usize {
                    encoding.push(false);
                }
            }
        }
    }
}

fn to_s6_chars(edges_encoding: Vec<bool>) -> String {
    if edges_encoding.len() % 6 != 0 {
        //
    }
    let mut s6_chars = String::new();
    let mut begin = 0;
    let mut end = 6;
    while begin < edges_encoding.len() {
        let mut char: u8 = 0;
        // encode next 6 bits into u8
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

pub fn encode_edges<E>(size: usize, edges: &Vec<E>) -> Vec<bool>
where
    E: Edge,
{
    let edge_encoding_size = edge_encoding_size(size);
    let mut v: usize = 0;
    let mut vec: Vec<bool> = Vec::new();
    for edge in edges {
        if edge.to() > (v + 1) {
            // shift v
            vec.push(false);
            vec.append(&mut bitvec_from_u64(edge.to() as u64, edge_encoding_size));
            v = edge.to();
        }
        if edge.to() == v + 1 {
            vec.push(true);
            v = v + 1;
        } else {
            vec.push(false);
        }
        vec.append(&mut bitvec_from_u64(edge.from() as u64, edge_encoding_size));
    }
    vec
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

pub fn edge_encoding_size(graph_size: usize) -> u8 {
    (graph_size as f64).log(2 as f64).ceil() as u8
}

fn edge_max_min_compare<E>(first: &E, second: &E) -> Ordering
where
    E: Edge,
{
    let max_first = cmp::max(first.from(), first.to());
    let max_second = cmp::max(second.from(), second.to());
    let compare_max = max_first.cmp(&max_second);
    if Ordering::Equal.eq(&compare_max) {
        let min_first = cmp::min(first.from(), first.to());
        let min_second = cmp::min(second.from(), second.to());
        return min_first.cmp(&min_second);
    }
    compare_max
}
