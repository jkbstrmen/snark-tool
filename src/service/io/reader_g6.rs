use std::str::Chars;

use crate::graph::graph;
use crate::service::io::error::ReadError;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::{Graph, Undirected};
use std::fs::File;
use std::io::{BufRead, Error};
use std::marker::PhantomData;
use std::{io, result};

pub const BIAS: u8 = 63;
pub const SMALLN: u64 = 62;

type Result<T> = result::Result<T, ReadError>;

// TODO - create graph reader struct
// implement function - next_graph - read_from_file - line by line or so

pub struct G6Reader<G>
where
    G: graph::Graph,
{
    _ph: PhantomData<G>,
}

impl<G> G6Reader<G>
where
    G: graph::Graph,
{
    pub fn new() -> Self {
        G6Reader { _ph: PhantomData }
    }

    pub fn read_by_lines(file: &File, count: u64) -> Result<Vec<G>> {
        let mut lines = io::BufReader::new(file).lines();

        let mut graphs = vec![];

        for i in 0..count {
            let line = lines.next();
            match line {
                None => {
                    // warn - file contains less graphs than specified to work with
                }
                Some(line) => {
                    println!("{:?}", line);

                    let graph = read_graph(line.unwrap());

                    // graphs.push(graph.unwrap());
                }
            }
        }

        Ok(graphs)
    }

    // pub fn read_graph(source: impl AsRef<str>) -> Result<G> {
    //     let mut iterator = source.as_ref().chars();
    //     let size = get_graph_size(&mut iterator);
    //     let graph = create_graph(&mut iterator, size? as u32);
    //
    //     // G::from_str();
    //
    //     Ok(graph)
    // }

    // fn create_graph(iterator: &mut Chars, size: u32) -> Result<G> {
    //     let nodes = size as usize;
    //     let edges = (size * 3 / 2) as usize;
    //     let mut undirected = StableGraph::<u8, u16, Undirected, u8>::with_capacity(nodes, edges);
    //
    //     for _node in 0..size {
    //         undirected.add_node(0);
    //     }
    //
    //     let error = "error";
    //     let mut char = iterator.next();
    //     let mut position = Position { row: 0, column: 1 };
    //     while char != None {
    //         let bits = format!("{:b}", (char.expect(error) as u8) - BIAS);
    //         for _i in 0..(6 - bits.len()) {
    //             position.increment();
    //         }
    //         for char in bits.chars() {
    //             if char == '1' {
    //                 undirected.add_edge(
    //                     NodeIndex::new(position.row),
    //                     NodeIndex::new(position.column),
    //                     0,
    //                 );
    //             }
    //             position.increment();
    //         }
    //         char = iterator.next();
    //     }
    //     undirected
    // }
}

pub fn read_graph(source: impl AsRef<str>) -> Result<StableGraph<u8, u16, Undirected, u8>> {
    let mut iterator = source.as_ref().chars();
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
        let bits = format!("{:b}", (char.expect(error) as u8) - BIAS);
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
pub struct Position {
    pub row: usize,
    pub column: usize,
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

pub fn get_graph_size(iterator: &mut Chars) -> Result<u64> {
    let mut char = iterator.next();
    if char == Some(':') || char == Some('&') {
        char = iterator.next();
    }

    if char.is_none() {
        return Err(ReadError {
            message: "".to_string(),
        });
    }

    let mut size = (char.unwrap() as u64) - BIAS as u64;
    if size > SMALLN {
        char = iterator.next();
        if char.is_none() {
            return Err(ReadError {
                message: "".to_string(),
            });
        }
        size = (char.unwrap() as u64) - BIAS as u64;

        if size > SMALLN {
            char = iterator.next();
            if char.is_none() {
                return Err(ReadError {
                    message: "".to_string(),
                });
            }
            size = (char.unwrap() as u64) - BIAS as u64;
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

fn append_char_binary_to_size(mut size: u64, iterator: &mut Chars) -> Result<u64> {
    let char = iterator.next();
    if char.is_none() {
        return Err(ReadError {
            message: "".to_string(),
        });
    }
    size = (size << 6) | ((char.unwrap() as u64) - BIAS as u64);
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
