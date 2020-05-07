use std::fs::File;
use std::io::{self, BufRead};

use crate::graph::graph::{Graph, GraphConstructor};
use crate::service::io::error::ReadError;
use crate::service::io::reader::Reader;
use std::{fs, marker, result};

type Result<T> = result::Result<T, ReadError>;

const WRONG_FORMAT: &str = "Wrong ba format";

pub struct BaReader<'a, G> {
    lines: io::Lines<io::BufReader<&'a fs::File>>,
    // TODO - point error to specific line
    // line: usize,
    graphs_count: Option<usize>,

    _ph: marker::PhantomData<G>,
}

impl<'a, G> Reader<'a, G> for BaReader<'a, G>
where
    G: Graph + GraphConstructor,
{
    fn new(file: &'a File) -> Self {
        BaReader {
            lines: io::BufReader::new(file).lines(),
            // line: 0,
            graphs_count: None,
            _ph: marker::PhantomData,
        }
    }

    fn next(&mut self) -> Option<Result<G>> {
        if self.graphs_count.is_none() {
            let count = self.get_graphs_count();
            match count {
                Ok(count_) => {
                    self.graphs_count = Some(count_);
                }
                Err(error) => {
                    return Some(Err(error));
                }
            }
        }
        let graph = self.read_graph_ba();
        graph.transpose()
    }
}

impl<'a, G> BaReader<'a, G>
where
    G: Graph + GraphConstructor,
{
    fn get_graphs_count(&mut self) -> Result<usize> {
        let graphs_count = self.next_numbers_vector()?;
        let count = graphs_count.get(0);
        if count.is_some() {
            return Ok(count.unwrap().clone());
        }
        Err(ReadError {
            message: "Wrong ba format - graphs count missing".to_string(),
        })
    }

    fn next_numbers_vector(&mut self) -> Result<Vec<usize>> {
        let mut vector = Vec::<usize>::new();
        let mut line = self.lines.next();

        while line.is_some() {
            if let Ok(line_str) = line.unwrap() {
                if line_str.trim().chars().next().unwrap() == '{' {
                    line = self.lines.next();
                    continue;
                }
                let mut split = line_str.split_whitespace();
                let mut next = split.next();
                while next.is_some() {
                    vector.push(next.unwrap().parse()?);
                    next = split.next();
                }
                if !vector.is_empty() {
                    return Ok(vector);
                }
            }
            line = self.lines.next();
        }
        Ok(vector)
    }

    fn get_serial_number(&mut self) -> Result<Option<usize>> {
        self.get_single_number()
    }

    fn get_size(&mut self) -> Result<usize> {
        let result = self.get_single_number();
        return match result {
            Ok(opt) => {
                if opt.is_some() {
                    return Ok(opt.unwrap());
                }
                Err(ReadError {
                    message: format!("{}: size not found", WRONG_FORMAT),
                })
            }
            Err(err) => Err(err),
        };
    }

    fn get_single_number(&mut self) -> Result<Option<usize>> {
        let mut vec = self.next_numbers_vector()?;
        if vec.len() == 1 {
            return Ok(vec.pop());
        }
        if vec.is_empty() {
            return Ok(None);
        }
        Err(ReadError {
            message: "asked for single number but get vector".to_string(),
        })
    }

    fn read_graph_ba(&mut self) -> Result<Option<G>> {
        let serial_number = self.get_serial_number()?;
        if serial_number.is_none() {
            return Ok(None);
        }
        let size = self.get_size()?;
        let edges = (size * 3 / 2) as usize;
        let mut graph = G::with_capacity(size, edges);
        for from in 0..size {
            let vec = self.next_numbers_vector()?;
            for to in vec.iter() {
                graph.add_edge(from, to.clone());
            }
        }
        Ok(Some(graph))
    }
}

pub fn read_preface_and_count(file: &File) -> Result<(usize, String)> {
    let mut lines = io::BufReader::new(file).lines();
    let mut comments = String::new();
    let mut line = lines.next();
    while line.is_some() {
        if let Ok(mut line_str) = line.unwrap() {
            let first_char = line_str.trim().chars().next();
            if first_char.is_some() && first_char.unwrap() != '{' {
                line_str = String::from(line_str.trim());
                let count = line_str.parse()?;
                return Ok((count, comments));
            }
            comments.push_str(line_str.as_ref());
            comments.push_str("\n");
            line = lines.next();
        } else {
            return Err(ReadError {
                message: format!("unknown read error"),
            });
        }
    }
    Ok((0, comments))
}
