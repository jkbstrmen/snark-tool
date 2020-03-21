use crate::service::io::reader_g6::read_graph;
use crate::service::io::writer_g6::{to_g6_size, write_graph};

use crate::service::io::reader_ba::read_graph_ba;
// use crate::service::io::writer_ba::{append_graph_ba_to_file, write_graph_ba};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[test]
fn should_read_g6() {
    // let graph_path = "resources/graphs/graphG6.g6";
    let graph_path = "resources/graphs/petersen.g6";

    if let Ok(lines) = read_lines(graph_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_str) = line {
                // println!("{}", line_str);
                // let error = "Wrong g6 format";
                let graph = read_graph(line_str.as_str());

                // print_graph(graph.unwrap());
                // assert
            }
        }
    }
}

#[test]
fn should_read_ba() {
    // let graph_path = "resources/graphs/5flow28.28";
    let graph_path = "resources/graphs/DSXDS.ALL";

    if let Ok(lines) = read_lines(graph_path) {
        // Consumes the iterator, returns an (Optional) String

        let graph = read_graph_ba(lines);
    }
}

#[test]
fn should_write_ba() {
    // move to test_data
    let graph_string =
        "]C??@Q??GCCA@??Bo??C@O?C?G_E????\\?O?A??H_??@C?@??_?C???g????G??B@??C????Ag";
    let graph_path = "resources/graphs/write_ba.ALL";

    let file_result = OpenOptions::new().write(true).append(true).open(graph_path);

    match file_result {
        Ok(mut file) => {
            let graph = read_graph(graph_string);
            // write graphs count
            writeln!(file, "1");

            // TODO
            // write_graph_ba(graph.unwrap(), 1, &mut file);

            // assert
        }
        _ => {}
    }
}

#[test]
fn should_append_ba() {
    let graph_path = "resources/graphs/append_ba.ALL";
    let graph_string =
        "]C??@Q??GCCA@??Bo??C@O?C?G_E????\\?O?A??H_??@C?@??_?C???g????G??B@??C????Ag";
    let graph = read_graph(graph_string);

    // TODO
    // append_graph_ba_to_file(graph.unwrap(), graph_path);

    // assert
}

#[test]
fn should_write_g6() {
    // move to test_data
    let graph_string =
        "]C??@Q??GCCA@??Bo??C@O?C?G_E????\\?O?A??H_??@C?@??_?C???g????G??B@??C????Ag";
    let graph = read_graph(graph_string);

    let mut w = Vec::new();
    write_graph(graph.unwrap(), &mut w);
    assert_eq!((graph_string.to_owned() + "\n").as_bytes(), &w[..]);
}

#[test]
fn should_code_size() {
    let res = to_g6_size(30);
    assert_eq!(res, "]");

    let res = to_g6_size(12345);
    assert_eq!(res, "~B?x");

    let res = to_g6_size(460175067);
    assert_eq!(res, "~~?ZZZZZ");
}

use crate::service::io::reader_s6::bitvec_to_u64;
use crate::service::io::writer_s6::{bitvec_from_u64, encode_edges, to_s6_string};
use crate::service::io::{reader_g6, reader_s6, writer_s6};
use bit_vec::BitVec;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::visit::EdgeRef;
use petgraph::Undirected;

type Graph = StableGraph<u8, u16, Undirected, u8>;

fn print_graph(graph: Graph) {
    for node_index in graph.node_indices() {
        print!("{}: ", node_index.index());
        for edge in graph.edges(node_index) {
            print!("{} ", edge.target().index());
        }
        println!();
    }
}

#[test]
fn should_write_s6() {
    let graph_path = "resources/graphs/petersen.g6";

    if let Ok(lines) = read_lines(graph_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_str) = line {
                println!("{}", line_str);
                // let error = "Wrong g6 format";
                let graph = read_graph(line_str.as_str());
                // print_graph(graph.unwrap());

                let mut w = Vec::new();
                writer_s6::write_graph(&graph.unwrap(), &mut w);

                let string = String::from_utf8(w).unwrap();
                println!("{:?}", string);
                // assert
            }
        }
    }
}

#[test]
fn should_read_graph_s6() {
    let petersen_s6 = ":IG?SPc_EOrOFCQN";
    let graph_res = reader_s6::read_graph(petersen_s6);

    // assert

    // let test_graph = ":Fa@x^";
    // let graph_res = reader_s6::read_graph(test_graph);

    // assert

    print_graph(graph_res.unwrap());

    //println!("{:?}", graph_res);
}

#[test]
fn should_read_graph_g6_from_string() {
    // let graph_g6 = "]?@G@U?OK?GP?CD?o???@G???AX??__????G???g_????CG???C???B_??GO??@PAA???A_??G";
    let graph_g6 = "]C@O?SAGC??P??O@o?Q?`????aGO????SK???O?O?OC???F??A??C??c???O@??@K???????@W";
    let graph_res = reader_g6::read_graph(graph_g6);
    print_graph(graph_res.unwrap());
}

// temp
#[test]
fn should_open_file() {
    let path = "test_test.txt";
    let file_result = OpenOptions::new().read(true).open(&path);

    println!("{:?}", file_result);
}
