use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::Undirected;
use std::fs::File;
use std::io::{self, BufRead};

// temp
use crate::graph::graph;
use crate::graph::graph::Graph;
use crate::service::io::error::ReadError;
use crate::service::io::reader::Reader;
use petgraph::visit::EdgeRef;
use std::{fs, marker, result};

type Result<T> = result::Result<T, ReadError>;

const WRONG_FORMAT: &str = "Wrong ba format";

pub struct BaReader<'a, G> {
    file: &'a fs::File,
    lines: io::Lines<io::BufReader<&'a fs::File>>,
    line: usize,
    graphs_count: Option<usize>,

    _ph: marker::PhantomData<G>,
}

impl<'a, G> Reader<'a, G> for BaReader<'a, G>
where
    G: graph::Graph,
{
    fn new(file: &'a File) -> Self {
        BaReader {
            file,
            lines: io::BufReader::new(file).lines(),
            line: 0,
            graphs_count: None,
            _ph: marker::PhantomData,
        }
    }

    fn next(&mut self) -> Option<Result<G>> {
        // TODO - point error to specific line

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
    G: Graph,
{
    fn get_graphs_count(&mut self) -> Result<usize> {
        let mut graphs_count = self.next_numbers_vector()?;
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
        // let vec = BaReader::<G>::next_numbers_vector(buffer)?;
        let mut vec = self.next_numbers_vector()?;
        let num = vec.get(0);
        if vec.len() == 1 {
            return Ok(vec.pop());
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
                // println!("Adding edge: ({}, {})", source, to);
                graph.add_edge(from, to.clone());
                // graph.update_edge(NodeIndex::new(from), NodeIndex::new(to.clone()), 0);
            }
        }
        Ok(Some(graph))
    }
}

// TODO - remove or refactor

pub fn get_graphs_count(mut buffer: io::Lines<io::BufReader<&File>>) -> Result<usize> {
    let mut graphs_count = next_numbers_vector_2(&mut buffer);

    let count = graphs_count.get(0);
    if count.is_some() {
        return Ok(count.unwrap().clone());
    }
    Err(ReadError {
        message: "Wrong ba format - graphs count missing".to_string(),
    })
}

pub fn get_graphs_count_with_preface(file: &File) -> Result<(usize, String)> {
    let mut lines = io::BufReader::new(file).lines();
    let count_preface_result = read_until_next_vector(&mut lines);

    if count_preface_result.is_ok() {
        let count_preface = count_preface_result.unwrap();
        let mut count_vec = count_preface.0;
        if count_vec.is_empty() {
            count_vec.push(0)
        }
        let count_opt = count_vec.get(0);
        if count_opt.is_some() {
            // count = vector.get(0).unwrap().clone();
            let tuple = (count_opt.unwrap().clone(), String::from(count_preface.1));
            return Ok(tuple);
        }
    }
    let message = WRONG_FORMAT.to_owned() + " - count of comments is wrong";
    Err(ReadError { message: message })

    // if empty file ...
}

pub fn read_graph_ba(mut buffer: io::Lines<io::BufReader<File>>) {
    // number of graphs in file
    let graphs_count = next_numbers_vector(&mut buffer);

    let serial_number = get_serial_number(&mut buffer);
    println!("serial number: {}", serial_number);

    let size = get_size(&mut buffer);
    println!("size: {}", size);

    let nodes = size as usize;
    let edges = (size * 3 / 2) as usize;
    let mut graph = StableGraph::<u8, u16, Undirected, u8>::with_capacity(nodes, edges);

    for _node in 0..size {
        graph.add_node(0);
    }

    for source in 0..size {
        let vec = next_numbers_vector(&mut buffer);
        // println!("{:?}", vec);

        for target in vec.iter() {
            println!("Adding edge: ({}, {})", source, target);
            graph.update_edge(NodeIndex::new(source), NodeIndex::new(target.clone()), 0);
        }
    }

    println!("Graph: {:?}", graph);

    // print graph
    for node_index in graph.node_indices() {
        print!("{}: ", node_index.index());

        for edge in graph.edges(node_index) {
            print!("{:?}, ", edge.target().index());
        }
        println!();
    }
}

fn get_serial_number(buffer: &mut io::Lines<io::BufReader<File>>) -> usize {
    get_size(buffer)
}

fn get_size(buffer: &mut io::Lines<io::BufReader<File>>) -> usize {
    next_numbers_vector(buffer).get(0).unwrap().clone()
}

// TODO - reimplement for BufReader of string?? ... next_numbers_vector_from_string
// TODO - handle errors
fn next_numbers_vector(buffer: &mut io::Lines<io::BufReader<File>>) -> Vec<usize> {
    let mut vector = Vec::<usize>::new();
    let mut par_level = 0;
    let mut line = buffer.next();

    while line.is_some() {
        if let Ok(line_str) = line.unwrap() {
            // actual line ++

            if line_str.trim().chars().next().unwrap() == '{' {
                line = buffer.next();
                continue;
            }

            // TODO handle errors

            let mut split = line_str.split_whitespace();
            let mut next = split.next();
            while next.is_some() {
                vector.push(next.unwrap().parse().unwrap());
                next = split.next();
            }
            if !vector.is_empty() {
                return vector;
            }
        }
        line = buffer.next();
    }
    vector
}

// ... next_numbers_vector_from_file
fn next_numbers_vector_2(buffer: &mut io::Lines<io::BufReader<&File>>) -> Vec<usize> {
    let mut vector = Vec::<usize>::new();
    let mut par_level = 0;
    let mut line = buffer.next();

    while line.is_some() {
        if let Ok(line_str) = line.unwrap() {
            if line_str.trim().chars().next().unwrap() == '{' {
                line = buffer.next();
                continue;
            }

            // TODO handle errors

            let mut split = line_str.split_whitespace();
            let mut next = split.next();
            while next.is_some() {
                vector.push(next.unwrap().parse().unwrap());
                next = split.next();
            }
            if !vector.is_empty() {
                return vector;
            }
        }
        line = buffer.next();
    }
    vector
}

fn read_until_next_vector(
    buffer: &mut io::Lines<io::BufReader<&File>>,
) -> Result<(Vec<usize>, String)> {
    let mut vector = Vec::<usize>::new();
    let mut comments = String::new();
    let mut line = buffer.next();
    while line.is_some() {
        if let Ok(line_str) = line.unwrap() {
            let first_char = line_str.trim().chars().next();
            if first_char.is_some() && first_char.unwrap() != '{' {
                let mut split = line_str.split_whitespace();
                let mut next = split.next();
                while next.is_some() {
                    // let ola = next.unwrap().parse().unwrap();
                    vector.push(next.unwrap().parse()?);
                    next = split.next();
                }
                if !vector.is_empty() {
                    return Ok((vector, comments));
                }
                break;
            }
            comments.push_str(line_str.as_ref());
            comments.push_str("\n");
            line = buffer.next();
        } else {
            let message = WRONG_FORMAT.to_owned() + " - count of comments is wrong";
            return Err(ReadError { message });
        }
    }
    Ok((vector, comments))

    // Err(ReadError {
    //     message: "".to_string(),
    // })
}
