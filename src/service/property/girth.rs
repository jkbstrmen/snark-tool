use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;
use crate::service::graph_traversal::cycle_discovery::BFSCyclehDiscovery;

pub fn girth<G: Graph>(graph: &G) -> usize {
    let mut shortest_cycle_length = usize::max_value();

    for vertex in graph.vertices() {
        let length_of_cycle = find_cycle(graph, vertex.index());
        if length_of_cycle < shortest_cycle_length {
            shortest_cycle_length = length_of_cycle;
        }
    }

    shortest_cycle_length
}

fn find_cycle<G: Graph>(graph: &G, vertex: usize) -> usize {
    let mut cd = BFSCyclehDiscovery::new(graph, vertex);
    cd.length_of_first_cycle()
}

///
/// TESTS
///
#[cfg(test)]
mod tests {
    use crate::graph::undirected::simple_graph::graph::SimpleGraph;
    use crate::service::io::reader::GraphFileReader;
    use crate::service::io::reader_g6::G6Reader;
    use crate::service::property::girth::girth;
    use crate::tests::test_data::test_data;
    use std::fs;

    #[test]
    fn should_have_girth_five() {
        let graph = test_data::get_petersen_graph();
        let girth = girth(&graph);
        assert_eq!(girth, 5);
    }

    #[test]
    fn should_have_girth_six() {
        let graph_string = test_data::SNARK_IN_G6_30_GIRTH_6;
        let graph = G6Reader::<SimpleGraph>::read_graph(graph_string).unwrap();
        let girth = girth(&graph);
        assert_eq!(girth, 6);
    }

    #[test]
    fn should_have_girth_five_from_file() {
        let path = test_data::GG_30_G05_CYC4_G6_FILE_PATH;
        let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();
        let mut reader = G6Reader::<SimpleGraph>::new(&file_result);

        let mut girth_6_counter = 0;

        while let Some(graph_result) = reader.next() {
            let graph = graph_result.unwrap();

            let girth = girth(&graph);

            if girth == 6 {
                girth_6_counter += 1;
            } else {
                assert_eq!(girth, 5);
            }
        }
        assert_eq!(girth_6_counter, 1);
    }
}
