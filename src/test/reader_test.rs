use crate::service::io::reader_g6::read_graph_g6;

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;
use crate::service::io::reader_ba::read_graph_ba;
use crate::service::io::writer_ba::append_graph_ba_to_file;

const BIAS: u32 = 63;

// TODO - handle ownership of File ... to be able ..

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new( file).lines())
}

#[test]
fn should_read_g6() {
    let graph_path = "resources/graphs/graphG6.g6";
    // let content = std::fs::read_to_string(graph_path).expect("could not read file");

    if let Ok(lines) = read_lines(graph_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_str) = line {
                println!("{}", line_str);

                let error = "Wrong g6 format";

                // let graph = read_graph_g6(content.as_str());
                let graph = read_graph_g6(line_str.as_str());
                // println!("{:?}", graph);


            }
        }
    }
}

#[test]
fn should_read_ba(){
    // let graph_path = "resources/graphs/5flow28.28";
    let graph_path = "resources/graphs/DSXDS.ALL";

    if let Ok(lines) = read_lines(graph_path) {
        // Consumes the iterator, returns an (Optional) String

        let graph = read_graph_ba(lines);

    }
}

#[test]
fn should_write_ba(){

    let graph_string = "]C??@Q??GCCA@??Bo??C@O?C?G_E????\\?O?A??H_??@C?@??_?C???g????G??B@??C????Ag";
    let graph_path = "resources/graphs/new_file.ALL";

    // let fileResult = File::create(graph_path);

    let fileResult = OpenOptions::new()
        .read(true)
        .write(true)
        .open(graph_path);

    match fileResult {
        Ok(mut file) => {

            let graph = read_graph_g6(graph_string);
            append_graph_ba_to_file(graph.unwrap(), &mut file);


        }
        _ => {}
    }

}
