use crate::service::io::reader;
use crate::service::io::reader::read_graph;

const BIAS: u32 = 63;

#[test]
fn it_works() {

    // rea

    let graph_path = "resources/graphs/graphG6.g6";
    let content = std::fs::read_to_string(graph_path).expect("could not read file");
    // println!("Graph file content: {}", content);

    // read_graph(content.as_str());

    // 3 0 57

    let num = 3;
    let nn = (num << 6) | 0;
    let nnn = (nn << 6) | 57;


    println!("{}", nnn);

    let n = 63 - BIAS;
    let n = (n << 6) | (90 - BIAS);
    let n = (n << 6) | (90 - BIAS);
    let n = (n << 6) | (90 - BIAS);
    let n = (n << 6) | (90 - BIAS);
    let n = (n << 6) | (90 - BIAS);
    println!("{}", n);

    // println!("Testing");
    // assert_eq!(2 + 2, 4);
}