use crate::service::io::reader_g6::read_graph;
use crate::service::io::writer_g6::{to_g6_size, write_graph};

use crate::service::io::reader_ba::read_graph_ba;
use crate::service::io::writer_ba::{append_graph_ba_to_file, write_graph_ba};
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
    let graph_path = "resources/graphs/graphG6.g6";

    if let Ok(lines) = read_lines(graph_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_str) = line {
                println!("{}", line_str);
                // let error = "Wrong g6 format";
                let graph = read_graph(line_str.as_str());

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

    let fileResult = OpenOptions::new().write(true).append(true).open(graph_path);

    match fileResult {
        Ok(mut file) => {
            let graph = read_graph(graph_string);
            // write graphs count
            writeln!(file, "1");
            write_graph_ba(graph.unwrap(), 1, &mut file);

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

    append_graph_ba_to_file(graph.unwrap(), graph_path);

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
