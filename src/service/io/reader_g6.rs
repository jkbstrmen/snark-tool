use std::str::Chars;

use crate::service::io::error::IoError;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::{Graph, Undirected};

// temp
use petgraph::visit::EdgeRef;

const BIAS: u64 = 63;
const SMALLN: u64 = 62;

// TODO - create graph reader struct
// implement function - next_graph


pub fn read_graph_g6(source: &str) -> Result<StableGraph<u8, u16, Undirected, u8>, IoError> {
    let mut iterator = source.chars();
    let size = get_graph_size(&mut iterator);
    let graph = create_graph(&mut iterator, size? as u32);

    // print graph
    // for node_index in graph.node_indices() {
    //     print!("{}: ", node_index.index());
    //
    //     for edge in graph.edges(node_index) {
    //         print!("{:?}, ", edge.target().index());
    //     }
    //     println!();
    // }

    Ok(graph)
}

fn create_graph(iterator: &mut Chars, size: u32) -> StableGraph<u8, u16, Undirected, u8> {
    let nodes = size as usize;
    let edges = (size * 3 / 2) as usize;
    let mut undirected = StableGraph::<u8, u16, Undirected, u8>::with_capacity(nodes, edges);

    for _node in 0..size {
        undirected.add_node(0);
    }

    let error = "error";
    let mut char = iterator.next();
    let mut position = Position { row: 0, column: 1 };
    while char != None {
        let bits = format!("{:b}", (char.expect(error) as u8) - BIAS as u8);
        for _i in 0..(6 - bits.len()) {
            position.increment();
        }
        for char in bits.chars() {
            if char == '1' {
                undirected.add_edge(
                    NodeIndex::new(position.row),
                    NodeIndex::new(position.column),
                    0,
                );
            }
            position.increment();
        }
        char = iterator.next();
    }
    undirected
}

#[derive(Debug)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    pub fn increment(&mut self) {
        if (self.row + 1) == self.column {
            self.column += 1;
            self.row = 0;
        } else {
            self.row += 1;
        }
    }
}

fn get_graph_size(iterator: &mut Chars) -> Result<u64, IoError> {
    let mut char = iterator.next();
    if char == Some(':') || char == Some('&') {
        char = iterator.next();
    }

    if char.is_none() { return Err(IoError {}); }

    let mut size = (char.unwrap() as u64) - BIAS;
    if size > SMALLN {
        char = iterator.next();
        if char.is_none() { return Err(IoError {}); }
        size = (char.unwrap() as u64) - BIAS;

        if size > SMALLN {
            char = iterator.next();
            if char.is_none() { return Err(IoError {}); }
            size = (char.unwrap() as u64) - BIAS;
            size = append_char_binary_to_size(size, iterator)?;
            size = append_char_binary_to_size(size, iterator)?;
            size = append_char_binary_to_size(size, iterator)?;
            size = append_char_binary_to_size(size, iterator)?;
            size = append_char_binary_to_size(size, iterator)?;
        } else {
            size = append_char_binary_to_size(size, iterator)?;
            size = append_char_binary_to_size(size, iterator)?;
        }
    }
    Ok(size)
}

fn append_char_binary_to_size(mut size: u64, iterator: &mut Chars) -> Result<u64, IoError> {
    let char = iterator.next();
    if char.is_none() {
        return Err(IoError {});
    }
    size = (size << 6) | ((char.unwrap() as u64) - BIAS);
    Ok(size)
}

fn petgraph_playground() {
    // let graph = Graph{
    //     nodes: vec![],
    //     edges: vec![],
    //     ty: PhantomData
    // };

    let mut undirected = StableGraph::<u8, u16, Undirected, u8>::with_capacity(10, 20);
    // let mut undirected = StableGraph::<u8, u16, Undirected, u8>::from(10, 20);

    undirected.add_node(0);
    undirected.add_node(0);
    undirected.add_node(0);
    undirected.add_edge(NodeIndex::new(1), NodeIndex::new(2), 0);
    println!("{:?}", undirected);

    undirected.remove_node(NodeIndex::new(1));
    println!("after removal: ");
    println!("{:?}", undirected);

    for node in undirected.node_indices() {
        let weight = undirected.node_weight(node);
        println!("{:?} -> weight: {:?}", node, weight);
    }

    // let mut undirected = UnGraph::with_capacity(10, 20);
    // println!("Undirected graph: {:?}", undirected);

    let mut graph = Graph::<u8, u8>::new();
    // graph.add_edge(0, 1);
    graph.add_node(1);

    //graph.

    // println!("Graph: {:?}", graph);
}
