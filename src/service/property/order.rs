use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;

// pub fn order<G: Graph>(graph: G) -> result::Result<usize, Error> {
//     Ok(0)
// }

#[allow(dead_code)]
pub fn is_cubic<G: Graph>(graph: G) -> bool {
    let mut is_cubic = true;
    for vertex in graph.vertices() {
        let edges = graph.edges_of_vertex(vertex.index());
        let mut edges_count = 0;
        for _edge in edges {
            edges_count += 1;
        }
        if edges_count != 3 {
            is_cubic = false;
        }
    }
    is_cubic
}

///
/// TESTS
///
#[cfg(test)]
mod tests {
    use crate::graph::undirected::simple_graph::graph::SimpleGraph;
    use crate::service::io::reader_g6::G6Reader;
    use crate::service::property::order::is_cubic;
    use crate::tests::test_data::test_data;

    #[test]
    fn should_be_cubic() {
        let graph = test_data::get_petersen_graph();
        let cubic = is_cubic(graph);
        assert_eq!(cubic, true);

        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_36_STABLE_RES_3).unwrap();
        let cubic = is_cubic(graph);
        assert_eq!(cubic, true);
    }
}
