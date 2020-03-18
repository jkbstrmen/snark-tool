use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::Undirected;
use std::fs::File;
use std::io::{self, BufRead};

// temp
use crate::service::io::error::ReadError;
use petgraph::visit::EdgeRef;

// implement function - next_graph

pub fn get_graphs_count(mut buffer: io::Lines<io::BufReader<&File>>) -> Result<usize, ReadError> {
    let mut graphs_count = next_numbers_vector_2(&mut buffer);

    let count = graphs_count.get(0);
    if count.is_some() {
        return Ok(count.unwrap().clone());
    }
    Err(ReadError {
        message: "Wrong ba format - graphs count missing".to_string(),
    })
}

const WRONG_FORMAT: &str = "Wrong ba format";

pub fn get_graphs_count_with_preface(file: &File) -> Result<(usize, String), ReadError> {
    let mut lines = io::BufReader::new(file).lines();
    let count_preface_result = read_until_next_vector(&mut lines);

    if count_preface_result.is_ok() {
        let count_preface = count_preface_result.unwrap();
        let count_opt = count_preface.0.get(0);
        if count_opt.is_some() {
            // count = vector.get(0).unwrap().clone();
            let tuple = (count_opt.unwrap().clone(), String::from(count_preface.1));
            return Ok(tuple);
        }
    }
    let message = WRONG_FORMAT.to_owned() + " - count of comments is wrong";
    Err(ReadError { message: message })
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
) -> Result<(Vec<usize>, String), ReadError> {
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
                    vector.push(next.unwrap().parse().unwrap());
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
    Err(ReadError {
        message: "".to_string(),
    })
}
