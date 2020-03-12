use crate::service::io::reader;
use crate::service::io::reader::read_graph;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const BIAS: u32 = 63;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[test]
fn it_works() {

    // rea

    let graph_path = "resources/graphs/graphG6.g6";
    let content = std::fs::read_to_string(graph_path).expect("could not read file");
    // println!("Graph file content: {}", content);

    // TODO - get line of file content

    if let Ok(lines) = read_lines(graph_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);

                read_graph(content.as_str().trim());
            }
        }
    }


    // read_graph(content.as_str());

    // 3 0 57

    // let num = 3;
    // let nn = (num << 6) | 0;
    // let nnn = (nn << 6) | 57;
    //
    //
    // println!("{}", nnn);
    //
    // let mut n = 63 - BIAS;
    // println!("{:b}", n);
    // n = (n << 6) | (90 - BIAS);
    // println!("{:b}", n);
    // n = (n << 6) | (90 - BIAS);
    // println!("{:b}", n);
    // n = (n << 6) | (90 - BIAS);
    // println!("{:b}", n);
    // n = (n << 6) | (90 - BIAS);
    // println!("{:b}", n);
    // n = (n << 6) | (90 - BIAS);
    // println!("{:b}", n);
    // println!("{}", n);
    //
    //
    //
    // let mut n = (90 - BIAS);
    // println!("{:b}", n);
    // n = n << 6;
    // println!("{:b}", n);
    // n = n | (90 - BIAS);
    // println!("{:b}", n);

    // println!("Testing");
    // assert_eq!(2 + 2, 4);
}