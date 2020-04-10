#[cfg(test)]
use crate::graph::traits::graph::Graph;
#[cfg(test)]
use crate::graph::undirected::simple_graph::SimpleGraph;
#[cfg(test)]
use crate::service::io::reader::Reader;
#[cfg(test)]
use crate::service::io::reader_g6::G6Reader;
#[cfg(test)]
use std::fs::OpenOptions;
#[cfg(test)]
use std::time::Instant;

#[test]
fn test() {
    let mut neighbors: [u8; 3] = [0, 1, 2];
    neighbors[0] = 1;

    println!("{:?}", neighbors);

    // let size = 5;
    // let mut neighbors: [u8; size] = [0; size];
}

#[test]
fn init_colouriser() {
    // snarks
    // let graph_str = "I?h]@eOWG"; // OK
    // let graph_str = "Ss??GOGA_I?c????GOQAACGO_P?_K@?S?"; // OK
    // let graph_str = "c?HI@cO?GC?@_AOCp????_G??C??@???O?O????O??`???H???C???_c?_??g??@??C????C?G??g?????CG??A?G?????w??@_?????g@"; // OK
    let graph_str = "g?`G?e?WG?D????A?@?????g??W?E???eA??D?????G???G???K???BC?O?????@?O??C?CO?IC????`????????O?????w?O??@????W??????B??????_?G@??G?@?G?A"; // OK

    // no snarks
    // let graph_str = "Qs?GOGA?OG@?CDGIAS@A_O@@?GG"; // OK
    // OK
    // let graph_str = "~?@os??GO????????????????H??W??C_?B???K???w??????????A???@?????????????G????O???????????????????????????????????????????????@??????@??????_??????O?????A??????G???????C??????A???@???C???C???G???C???C???A???A????_???O???O???A???C???G????C???C????C???@????A????O????O????_???G???@?????_???A????C????O????O????C????_???C????_???O?????A????C????A????O?????@????G?????O????_?????G???@??????A????@??????_???@??????????C@??????????A?A?????????A?O?????????@@??????????GA???????????A@???????????__??????????_G??????????????C?_???????????C@??????????@?C????????????OC??????????@?C????????????c????????????CA????????????A?O??????A???@?????????G???G??????????@??A??????????@???A??????????C????G?????????@???A??????????G????O??????????A???G???????????????????C@??????????????GA??????????????GA??????????????C@??????????????@?O??????????????A?_??????????????_G??????????????C@?????????????W????????????????@G????????????????W????????????????o????A???????????????W?C???????????????W?O???????????????H?G???????????????E?A????????????????Q?_???????????????B?";

    let graph = G6Reader::<SimpleGraph>::read_graph(graph_str).unwrap();

    // println!("{}", graph);

    let mut matrix: Vec<u8> = vec![];

    for row in 0..graph.size() {
        for column in 0..graph.size() {
            if graph.has_edge(row, column) {
                matrix.push(1);
            } else {
                matrix.push(0);
            }
        }
    }

    // let colour = BFSColourGraph::is_colorable(matrix, graph.size() as u8);
    // println!("graph is colorable: {}", colour);
}

#[cfg(test)]
fn is_colourable(graph: &SimpleGraph) -> bool {
    let mut matrix: Vec<u8> = vec![];
    for row in 0..graph.size() {
        for column in 0..graph.size() {
            if graph.has_edge(row, column) {
                matrix.push(1);
            } else {
                matrix.push(0);
            }
        }
    }
    // let colour = BFSColourGraph::is_colorable(matrix, graph.size() as u8);
    // colour
    false
}

#[test]
fn measure_time() {
    let begin = Instant::now();

    // let path = "resources/graphs/Generated_2100_36vert_snarks.g6";
    let path = "resources/graphs/Generated_100_36vert_snarks.g6";
    let file_result = OpenOptions::new().read(true).open(&path).unwrap();

    let mut reader = G6Reader::<SimpleGraph>::new(&file_result);

    let mut all_false = true;
    let mut graph = reader.next();
    let mut index = 0;
    while graph.is_some() {
        println!("{}", index);
        if let Ok(graph) = graph.unwrap() {
            let result = is_colourable(&graph);
            if result {
                all_false = false;
            }
        }

        index += 1;
        graph = reader.next();
    }

    println!("all false: {}", all_false);
    println!("elapsed: {}[ms]", begin.elapsed().as_millis());
}

// fn print(graph: &Vec<u8>, graph_size: u8) {
//     for row in 0..graph_size {
//         for column in 0..graph_size {
//             let index: usize = (row as usize) * graph_size as usize + column as usize;
//             print!("{} ", graph.get(index as usize).unwrap());
//         }
//         println!();
//     }
// }

#[test]
fn test_bfs_v2() {
    // no snarks
    // let graph_str = "Qs?GOGA?OG@?CDGIAS@A_O@@?GG"; // OK
    let graph_str = "~?@os??GO????????????????H??W??C_?B???K???w??????????A???@?????????????G????O???????????????????????????????????????????????@??????@??????_??????O?????A??????G???????C??????A???@???C???C???G???C???C???A???A????_???O???O???A???C???G????C???C????C???@????A????O????O????_???G???@?????_???A????C????O????O????C????_???C????_???O?????A????C????A????O?????@????G?????O????_?????G???@??????A????@??????_???@??????????C@??????????A?A?????????A?O?????????@@??????????GA???????????A@???????????__??????????_G??????????????C?_???????????C@??????????@?C????????????OC??????????@?C????????????c????????????CA????????????A?O??????A???@?????????G???G??????????@??A??????????@???A??????????C????G?????????@???A??????????G????O??????????A???G???????????????????C@??????????????GA??????????????GA??????????????C@??????????????@?O??????????????A?_??????????????_G??????????????C@?????????????W????????????????@G????????????????W????????????????o????A???????????????W?C???????????????W?O???????????????H?G???????????????E?A????????????????Q?_???????????????B?";

    // snarks
    // let graph_str = "I?h]@eOWG"; // OK
    // let graph_str = "Ss??GOGA_I?c????GOQAACGO_P?_K@?S?"; // OK
    // let graph_str = "c?HI@cO?GC?@_AOCp????_G??C??@???O?O????O??`???H???C???_c?_??g??@??C????C?G??g?????CG??A?G?????w??@_?????g@"; // OK
    // let graph_str = "g?`G?e?WG?D????A?@?????g??W?E???eA??D?????G???G???K???BC?O?????@?O??C?CO?IC????`????????O?????w?O??@????W??????B??????_?G@??G?@?G?A"; // OK

    let graph = G6Reader::<SimpleGraph>::read_graph(graph_str).unwrap();

    // let graph_size = graph.size();

    let result1 = is_colourable(&graph);
    // let result2 = BFSColourGraphV2::is_colorable(&graph);

    println!("is colorable 1: {}", result1);
    // println!("is colorable 2: {}", result2);
}
