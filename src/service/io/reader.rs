
use petgraph::{Graph, Undirected};
use petgraph::graph::{UnGraph, NodeIndex};
use petgraph::stable_graph::StableGraph;

use bit_set::BitSet;
use bit_vec::BitVec;
use std::str::Chars;
// use bit_set::BitVec;

// 30
// ]C??@Q??GCCA@??Bo??C@O?C?G_E????\?O?A??H_??@C?@??_?C???g????G??B@??C????Ag

// 12345
// ~B?x]C??@Q??GCCA@??Bo??C@O?C?G_E????\?O?A??H_??@C?@??_?C???g????G??B@??C????Ag

// 460175067
// 126 126 63 90 90 90 90 90
// ~~?ZZZZZC??@Q??GCCA@??Bo??C@O?C?G_E????\?O?A??H_??@C?@??_?C???g????G??B@??C????Ag


pub fn read_graph(source: &str){

    println!("Graph source: {}", source);

    // as_num('&');
    // as_num(':');
    // as_num(']');
    // as_num('C');
    //
    // as_char(126);
    // as_char(63);
    // as_char(90);
    // as_char(120);
    //
    // as_binary(67-63);
    // as_binary(120);

    let mut iterator = source.chars();
    let size = get_graph_size(&mut iterator);
    println!("Graph size: {}", size);

    continue_reading(&mut iterator, size);

    // continue reading graph with iterator ...


    // let mut n = format!("{:b}", 66-63);
    // println!("Binary size: {}", n);
    // n.push_str(format!("{:b}", 63-63).as_str());
    // println!("Binary size: {}", n);
    // n.push_str(format!("{:b}", 120-63).as_str());
    //
    // println!("Binary size: {}", n);

}

fn continue_reading(iterator: &mut Chars, size: u32){

    // println!("Iterator left: {:?}", iterator.next());

    let nodes = size as usize;
    let edges = (size * 3 / 2) as usize;
    println!("Nodes: {}, edges: {}", nodes, edges);

    // println!("usize min: {}, usize max: {}", usize::min_value(), usize::max_value());
    // println!("u64 min: {}, u64 max: {}", u64::min_value(), u64::max_value());

    let mut undirected = StableGraph::<u8, u16, Undirected, u8>::with_capacity(nodes, edges);
    // let mut undirected = StableGraph::<u8, u16, Undirected, u8>::from(10, 20);

    for node in 0..size {
        undirected.add_node(0);
    }

    println!("Graph: {:?}", undirected);

    // TODO - format chars in iterator to binary form

    let error = "error";

    let mut char = iterator.next();
    let mut binary =  (char.expect(error) as u128) - BIAS as u128;
    println!("{:b}", binary);
    while char != None {

        let bits = format!("{:b}", (char.expect(error) as u8) - BIAS as u8);

        // add position specifier
        add_edges(&bits, &mut undirected);

        char = iterator.next();
    }

    // TODO - complete graph with missing zeroes?? - or not needed
}

struct Position{
    row: u32,
    column: u32
}

fn add_edges(bits: &String, graph: &mut StableGraph<u8, u16, Undirected, u8>){

    for i in 0..(6 - bits.len()) {
        print!("0 ");

        // increment rows and columns (edge position)

    }

    for char in bits.chars() {
        print!("{} ", char);
    }
    println!("bits: {}, size: {}", bits, bits.len());

}

fn add_batch(){

    // add at most 21 chars

}


fn as_char(num: u8){
    let num_c = num as char;
    println!("number: {}, as char: {}", num, num_c);
}

fn as_num(c: char) {
    let a_num = c as u8;
    println!("char: {}, as number: {}", c, a_num);
}

fn as_binary(num: u8){
    let binary = format!("{:b}", num);
    println!("number: {}, as bites: {}", num, binary);
}

const BIAS: u32 = 63;
const SMALLN: u32 = 62;

// TODO - return result instead of u32 - error when wrong g6 format

fn get_graph_size(iterator: &mut Chars) -> u32 {

    // BitSet::from_bit_vec();
    // BitVe
    // sizeTemp(source);

    // source.char_indices()
    let error = "Wrong g6 format";

    let mut char = iterator.next();

    if char == Some(':') || char == Some('&') {
        char = iterator.next();
    }

    let sizeChar = char.expect(error);
    let mut size = (sizeChar as u32) - BIAS;
    if  size > SMALLN {
        char = iterator.next();
        size =  (char.expect(error) as u32) - BIAS;

        if size > SMALLN {
            size =  (iterator.next().expect(error) as u32) - BIAS;
            size = (size << 6) | ((iterator.next().expect(error) as u32) - BIAS);
            size = (size << 6) | ((iterator.next().expect(error) as u32) - BIAS);
            size = (size << 6) | ((iterator.next().expect(error) as u32) - BIAS);
            size = (size << 6) | ((iterator.next().expect(error) as u32) - BIAS);
            size = (size << 6) | ((iterator.next().expect(error) as u32) - BIAS);
        } else {
            size = (size << 6) | ((iterator.next().expect(error) as u32) - BIAS);
            size = (size << 6) | ((iterator.next().expect(error) as u32) - BIAS);
        }

    }
    size

    // if char == Some('~') {
    //     char = iterator.next();
    //     if char == Some('~') {
    //
    //         println!("Biggest");
    //         return 0;
    //     }
    //
    //     handle_medium_size(char, iterator);
    //
    //
    //     println!("Medium");
    //     return 0;
    // }
    //
    // println!("Small");
    // let size = ((char.expect("Wrong G6 format") as u32) - BIAS);
    // size




    // let mut size: u32 = 0;
    // for char in source.chars(){
    //     if char == ':' || char == '&' {
    //         continue;
    //     }
    //     if char == '~' {
    //         // char.next().expect("wrong G6 format");
    //     }
    //
    //     size = ((char as u8) - BIAS) as u32;
    //
    //     // if first char is '~'  ... 63 <= size <= 258047
    //     // take first 4 chars
    //
    //     // if second char is '~'  ... 258048 <= n <= 68719476735
    //     // take first 8 chars
    //
    //     if size > SMALLN as u32 {
    //         return 0;
    //     }
    //     return size
    // }
    // size

    //
    // let bin_idx = "000000011011011011011011011011011011";
    // let intval = u64::from_str_radix(bin_idx, 2).unwrap();
    // println!("{}", intval);
}

fn handle_medium_size(char: Option<char>, mut iterator: Chars){

    let siz = u32::from_be_bytes([3, 0, 57, 0]);
    println!("============ {}", siz);

    let str = "B?x";
    let mut bytes = str.as_bytes();
    // for byte in bytes.iter_mut(){
    //     // byte - BIAS;
    //     // byte.min(BIAS.clone() as u8);
    // }
    let vec = BitVec::from_bytes(bytes);
    println!("BitVec: {:?}", vec);
    println!("BitVec: {:?}", vec.to_bytes());
    // let intval = u64::from_str_radix(vec, 2).unwrap();


    let c = char.expect("Wrong G6 format");
    char_to_binary(c);
    let c = ((c as u8) - BIAS as u8) as char;
    let mut cc = String::new();
    cc.push(c);
    let ccc = cc.as_bytes();
    let mut vec = BitVec::from_bytes(ccc);
    println!("BitVec: {:?}", vec);
    // vec.truncate(6);
    // vec = &vec[2..];
    // println!("Slice: {}", vec[0..2]);
    println!("Length: {}", vec.len());


    println!("BitVec: {:?}", vec);

    let mut binary = char_to_binary(char.expect("Wrong G6 format"));

    let bytes = binary.as_bytes();
    println!("Bytes: {:?}", bytes);

    let vec = BitVec::from_bytes(bytes);
    println!("BitVec: {:?}", vec);




    // complete_binary(&mut binary);
    // println!("Binary: {}", binary);

    println!("Char: {:?}", char);
    let char = iterator.next();

    println!("Char: {:?}", char);
    let char = iterator.next();
    // binary = char_to_binary(char.expect("Wrong G6 format"));

    println!("Char: {:?}", char);
    // let char = iterator.next();

    // println!("Char: {:?}", char);
    // let binary = char_to_binary(char.expect("Wrong G6 format"));
    // println!("Binary length: {}", binary.len());

}

fn complete_binary(binary: &mut String){

    // let mut complement = String::from();
    //
    // complement.push_str(binary);
    // // binary = complement;
    //
    // binary.as_bytes();


}

fn char_to_binary(c: char) -> String {
    let num = c as u32;
    println!("{:b}", num - BIAS);
    format!("{:b}", num - BIAS)
}


fn petgraph_playground(){

    // let graph = Graph{
    //     nodes: vec![],
    //     edges: vec![],
    //     ty: PhantomData
    // };

    let mut undirected = StableGraph::<u8, u16, Undirected, u8>::with_capacity(10, 20);
    // let mut undirected = StableGraph::<u8, u16, Undirected, u8>::from(10, 20);

    undirected.add_node(0);
    undirected.add_node(0);
    undirected.add_node(0);
    undirected.add_edge(NodeIndex::new(1), NodeIndex::new(2), 0);
    println!("{:?}", undirected);

    undirected.remove_node(NodeIndex::new(1));
    println!("after removal: ");
    println!("{:?}", undirected);

    for node in undirected.node_indices() {
        let weight = undirected.node_weight(node);
        println!("{:?} -> weight: {:?}", node, weight);
    }



    // let mut undirected = UnGraph::with_capacity(10, 20);
    // println!("Undirected graph: {:?}", undirected);


    let mut graph = Graph::<u8, u8>::new();
    // graph.add_edge(0, 1);
    graph.add_node(1);

    //graph.

    // println!("Graph: {:?}", graph);

}
