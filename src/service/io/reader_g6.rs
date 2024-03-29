use std::str::Chars;

use crate::graph::graph::{Graph, GraphConstructor};
use crate::service::io::error::ReadError;
use crate::service::io::reader::GraphFileReader;
use std::io::{BufRead, BufReader};
use std::marker::PhantomData;
use std::{fs, io, result};

pub const BIAS: u8 = 63;
pub const SMALLN: u64 = 62;
const WRONG_FORMAT: &str = "Wrong g6 format";

type Result<T> = result::Result<T, ReadError>;

pub struct G6Reader<'a, G>
where
    G: Graph,
{
    lines: io::Lines<BufReader<&'a fs::File>>,
    _ph: PhantomData<G>,
}

impl<'a, G> GraphFileReader<'a, G> for G6Reader<'a, G>
where
    G: Graph + GraphConstructor,
{
    fn new(file: &'a fs::File) -> Self {
        G6Reader {
            lines: io::BufReader::new(file).lines(),
            _ph: PhantomData,
        }
    }

    fn next(&mut self) -> Option<Result<G>> {
        let line_opt = self.lines.next();
        match line_opt {
            None => {
                // warn - file contains less graphs than specified to work with
                return None;
            }
            Some(line) => {
                if let Ok(line_str) = line {
                    if line_str.trim().is_empty() {
                        return self.next();
                    }
                    let graph = G6Reader::read_graph(line_str);
                    return Some(graph);
                } else {
                    // skip error lines?
                    return self.next();
                }
            }
        }
    }
}

impl<'a, G> G6Reader<'a, G>
where
    G: Graph + GraphConstructor,
{
    pub fn read_graph(source: impl AsRef<str>) -> Result<G> {
        let mut iterator = source.as_ref().chars();
        let size = get_graph_size(&mut iterator);
        let graph = G6Reader::create_graph(&mut iterator, size? as u32)?;
        Ok(graph)
    }

    fn create_graph(iterator: &mut Chars, size: u32) -> Result<G> {
        let vertices = size as usize;
        let edges = (size * 3 / 2) as usize;
        let mut graph = G::with_capacity(vertices, edges);
        let mut char = iterator.next();
        let mut position = Position { row: 0, column: 1 };
        while char != None {
            let char_num = G6Reader::<G>::extract_char(char.unwrap())?;
            let bits = format!("{:b}", char_num);
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

    fn extract_char(char: char) -> Result<u8> {
        let char_num = char as u8;
        if char_num < BIAS || char_num > BIAS * 2 {
            return Err(ReadError {
                message: format!(
                    "{} `{}` is not allowed character",
                    WRONG_FORMAT, char_num as char
                ),
            });
        }
        Ok(char_num - BIAS)
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
