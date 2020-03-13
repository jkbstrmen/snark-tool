use std::fs::File;
use std::io::{self, BufRead};
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::{Graph, Undirected};

// temp
use petgraph::visit::EdgeRef;
use crate::service::io::error::IoError;

// TODO - create graph reader struct
// implement function - next_graph

// TODO - try with &File ...

pub fn get_graphs_count(mut buffer: io::Lines<io::BufReader<File>>) -> Result<usize, IoError> {
    let mut graphs_count = next_numbers_vector(&mut buffer);
    // let ola = graphs_count.remove(0);
    let count = graphs_count.get(0);
    if count.is_some() {
        return Ok(count.unwrap().clone());
    }
    Err(IoError{})
}

// read one graph from file where is just one graph
pub fn read_graph_ba(mut buffer: io::Lines<io::BufReader<File>>){

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
            graph.update_edge(
                NodeIndex::new(source),
                NodeIndex::new(target.clone()),
                0,
            );
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
            if !vector.is_empty() { return vector; }
        }
        line = buffer.next();
    }
    vector
}

