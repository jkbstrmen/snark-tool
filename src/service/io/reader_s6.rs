use crate::service::io::error::ReadError;
use crate::service::io::reader_g6::{get_graph_size, BIAS};
use crate::service::io::writer_s6::{bitvec_from_u64, edge_encoding_size};
use petgraph::stable_graph::{NodeIndex, StableGraph};
use petgraph::Undirected;
use std::slice::Iter;
use std::str::Chars;

type Graph = StableGraph<u8, u16, Undirected, u8>;

struct S6Reader {
    edge_encoding_size: u8,
}

impl S6Reader {}

pub fn read_graph(source: impl AsRef<str>) -> Result<Graph, ReadError> {
    let string = String::from(source.as_ref());

    let mut chars = string.chars();
    let char_opt = chars.next();
    if char_opt.is_none() || !':'.eq(&char_opt.unwrap()) {
        return Err(ReadError {
            message: "".to_string(),
        });
    }
    let size = get_graph_size(&mut chars)?;
    let graph = create_graph(&mut chars, size)?;
    Ok(graph)
}

fn create_graph(iterator: &mut Chars, size: u64) -> Result<Graph, ReadError> {
    let nodes = size as usize;
    // reserve edges - in default for cubic graph
    let edges = (size * 3 / 2) as usize;
    let mut graph = StableGraph::<u8, u16, Undirected, u8>::with_capacity(nodes, edges);

    for _node in 0..size {
        graph.add_node(0);
    }
    let mut bit_vec = chars_to_bit_vector(iterator)?;

    let edge_encoding_size = edge_encoding_size(graph.node_count());
    discard_additional_bits(&mut bit_vec, edge_encoding_size);
    graph = decode_edges(&bit_vec, graph)?;
    Ok(graph)
}

fn discard_additional_bits(bits: &mut Vec<bool>, edge_encoding_size: u8) {
    let additional_bits = bits.len() % (edge_encoding_size + 1) as usize;
    for _additional_bit in 0..additional_bits {
        bits.pop();
    }
}

fn chars_to_bit_vector(chars: &mut Chars) -> Result<Vec<bool>, ReadError> {
    let mut char_opt = chars.next();
    let mut vec = vec![];
    while char_opt.is_some() {
        let num = (char_opt.unwrap() as u64) - BIAS as u64;
        let mut char_bitvec = bitvec_from_u64(num, 6);
        vec.append(&mut char_bitvec);
        char_opt = chars.next();
    }
    Ok(vec)
}

fn decode_edges(bits: &Vec<bool>, mut graph: Graph) -> Result<Graph, ReadError> {
    let edge_encoding_size = edge_encoding_size(graph.node_count());
    let size = graph.node_count();
    let mut v: usize = 0;

    let mut bit_iter = bits.iter();
    let mut bit_opt = bit_iter.next();
    while bit_opt.is_some() {
        let leading = bit_opt.unwrap();
        if *leading {
            v += 1;
        }

        let num = bitvec_to_u64(&mut bit_iter, edge_encoding_size)?;
        if num >= size {
            break;
        }
        if num > v {
            v = num;
        } else {
            graph.update_edge(NodeIndex::new(num), NodeIndex::new(v), 0);
        }
        bit_opt = bit_iter.next();
    }
    Ok(graph)
}

pub fn bitvec_to_u64(bit_iter: &mut Iter<bool>, encoding_size: u8) -> Result<usize, ReadError> {
    let mut begin: u8 = 0;
    let mut x: usize = 0;
    loop {
        begin += 1;
        if begin > encoding_size {
            break;
        }
        let bit = bit_iter.next();
        if bit.is_none() {
            return Err(ReadError {
                message: "wrong s6 format - missing bits".to_string(),
            });
        }
        x = (x << 1) | bit.unwrap().clone() as usize;
    }
    Ok(x)
}
