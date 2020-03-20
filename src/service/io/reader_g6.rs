use std::str::Chars;

use crate::graph::graph;
use crate::service::io::error::ReadError;
use crate::service::io::reader::Reader;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::{Graph, Undirected};
use std::io::{BufRead, BufReader, Error};
use std::marker::PhantomData;
use std::{fs, io, result};

pub const BIAS: u8 = 63;
pub const SMALLN: u64 = 62;

type Result<T> = result::Result<T, ReadError>;

pub struct G6Reader<'a, G>
where
    G: graph::Graph,
{
    file: &'a fs::File,
    lines: io::Lines<BufReader<&'a fs::File>>,
    _ph: PhantomData<G>,
}

impl<'a, G> Reader<'a, G> for G6Reader<'a, G>
where
    G: graph::Graph,
{
    fn new(file: &'a fs::File) -> Self {
        G6Reader {
            file,
            lines: io::BufReader::new(file).lines(),
            _ph: PhantomData,
        }
    }

    fn next(&mut self) -> Option<Result<G>> {
        let line = self.lines.next();
        match line {
            None => {
                // warn - file contains less graphs than specified to work with
                return None;
            }
            Some(line) => {
                if line.is_ok() {
                    let graph = G6Reader::read_graph(line.unwrap());
                    // graphs.push(graph.unwrap());
                    return Some(graph);
                }
            }
        }
        None
        // Err(ReadError{ message: "".to_string() })
    }
}

impl<'a, G> G6Reader<'a, G>
where
    G: graph::Graph,
{
    // pub fn new(file: &'a fs::File) -> Self {
    //     G6Reader {
    //         file,
    //         lines: io::BufReader::new(file).lines(),
    //         _ph: PhantomData,
    //     }
    // }
    //
    // pub fn next(&mut self) -> Option<Result<G>> {
    //     let line = self.lines.next();
    //     match line {
    //         None => {
    //             // warn - file contains less graphs than specified to work with
    //             return None;
    //         }
    //         Some(line) => {
    //             if line.is_ok() {
    //                 let graph = G6Reader::read_graph(line.unwrap());
    //                 // graphs.push(graph.unwrap());
    //                 return Some(graph);
    //             }
    //         }
    //     }
    //     None
    //     // Err(ReadError{ message: "".to_string() })
    // }

    // pub fn new() -> Self {
    //     G6Reader { _ph: PhantomData }
    // }

    pub fn read_by_lines(/*&self,*/ file: &fs::File, count: u64) -> Result<Vec<G>> {
        let mut lines = io::BufReader::new(file).lines();

        let mut graphs = vec![];

        for i in 0..count {
            let line = lines.next();
            match line {
                None => {
                    // warn - file contains less graphs than specified to work with
                }
                Some(line) => {
                    if line.is_ok() {
                        let graph = G6Reader::read_graph(line.unwrap());
                        graphs.push(graph.unwrap());
                    }
                }
            }
        }

        Ok(graphs)
    }

    fn read_graph(source: impl AsRef<str>) -> Result<G> {
        let mut iterator = source.as_ref().chars();
        let size = get_graph_size(&mut iterator);
        let graph = G6Reader::create_graph(&mut iterator, size? as u32)?;
        Ok(graph)
    }

    fn create_graph(iterator: &mut Chars, size: u32) -> Result<G> {
        let vertices = size as usize;
        let edges = (size * 3 / 2) as usize;
        let mut graph = G::with_capacity(vertices, edges);

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
                    graph.add_edge(position.row, position.column);
                }
                position.increment();
            }
            char = iterator.next();
        }
        Ok(graph)
    }
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

// TODO - to remove
pub fn read_graph(source: impl AsRef<str>) -> Result<StableGraph<u8, u16, Undirected, u8>> {
    let mut iterator = source.as_ref().chars();
    let size = get_graph_size(&mut iterator);
    let graph = create_graph(&mut iterator, size? as u32);
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
