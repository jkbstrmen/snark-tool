use petgraph::stable_graph::StableGraph;
use petgraph::Undirected;
use crate::service::io::error::IoError;
use crate::service::io::reader_g6::get_graph_size;
use std::str::Chars;

type Graph = StableGraph<u8, u16, Undirected, u8>;

pub fn read_graph(source: impl AsRef<str>) -> Result<Graph, IoError> {

    let string = String::from(source.as_ref());

    println!("{}", string);

    let mut chars = string.chars();
    let char_opt = chars.next();
    if char_opt.is_none() || !':'.eq(&char_opt.unwrap()) {
        return Err(IoError{});
    }

    let size = get_graph_size(&mut chars)?;
    let graph = create_graph(&mut chars, size);

    Err(IoError{})
}

fn create_graph(iterator: &mut Chars, size: u64) -> Result<Graph, IoError> {
    let nodes = size as usize;
    let edges = (size * 3 / 2) as usize;
    let mut graph = StableGraph::<u8, u16, Undirected, u8>::with_capacity(nodes, edges);

    for _node in 0..size {
        graph.add_node(0);
    }

    let error = "error";
    let mut char = iterator.next();

    // convert chars to bit vector
    let bit_vec = chars_to_bit_vector(iterator)?;

    // read bit vector by k-size slices and add edges
    graph = add_edges_from_bits(&bit_vec, graph)?;

    Ok(graph)
}

fn chars_to_bit_vector(chars: &mut Chars) -> Result<Vec<bool>, IoError> {

    // TODO

    Err(IoError{})
}

fn add_edges_from_bits(bits: &Vec<bool>, mut graph: Graph) -> Result<Graph, IoError> {

    // TODO

    // Err(IoError{})
    Ok(graph)
}

