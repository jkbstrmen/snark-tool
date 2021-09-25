use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;
use crate::service::colour::colouriser::Colouriser;
use std::marker;

pub struct Resistance<G, C>
where
    G: Graph + Clone,
    C: Colouriser,
{
    _g: marker::PhantomData<G>,
    _colourizer: C,
}

impl<G, C> Resistance<G, C>
where
    G: Graph + Clone,
    C: Colouriser,
{
    pub fn new_with_colouriser(colourizer: C) -> Self {
        Resistance {
            _g: marker::PhantomData,
            _colourizer: colourizer,
        }
    }

    pub fn edge_resistance(&self, graph: &G) -> Option<usize> {
        for i in 0..graph.size() {
            let e_res = self.edge_resistance_recursive(graph, i);
            if e_res.is_some() {
                return e_res;
            }
        }
        None
    }

    // optimization opportunity - do not copy graph - use mutable pointer
    fn edge_resistance_recursive(&self, graph: &G, max_nesting: usize) -> Option<usize> {
        if max_nesting == 0 {
            let colourable = C::is_colorable(graph);
            if colourable {
                return Some(0);
            }
            return None;
        }

        let mut local_graph = graph.clone();
        for edge in graph.edges() {
            local_graph.remove_edge(edge.from(), edge.to());
            let e_res = self.edge_resistance_recursive(&local_graph, max_nesting - 1);

            if e_res.is_some() {
                return Some(e_res.unwrap() + 1);
            }
            local_graph.add_edge(edge.from(), edge.to());
        }
        None
    }

    pub fn vertex_resistance(&self, graph: &G) -> Option<usize> {
        for i in 0..graph.size() {
            let v_res = self.vertex_resistance_recursive(graph, i);
            if v_res.is_some() {
                return v_res;
            }
        }
        None
    }

    fn vertex_resistance_recursive(&self, graph: &G, max_nesting: usize) -> Option<usize> {
        if max_nesting == 0 {
            let colourable = C::is_colorable(graph);
            if colourable {
                return Some(0);
            }
            return None;
        }

        for vertex in graph.vertices() {
            let mut local_graph = graph.clone();
            local_graph.remove_edges_of_vertex(vertex.index());
            let v_res = self.vertex_resistance_recursive(&local_graph, max_nesting - 1);
            if v_res.is_some() {
                return Some(v_res.unwrap() + 1);
            }
            // eventually add removed edges of vertex
        }
        None
    }
}

///
/// TESTS
///
#[cfg(test)]
mod tests {
    use crate::graph::undirected::simple_graph::graph::SimpleGraph;
    use crate::service::chromatic_properties::resistance::Resistance;
    use crate::service::colour::colouriser::Colouriser;
    use crate::service::colour::recursive::dfs_improved::DFSColourizer;
    use crate::service::io::reader_g6::G6Reader;
    use crate::tests::test_data::test_data;

    #[test]
    fn should_have_resistance_zero() {
        let res_tester = Resistance::new_with_colouriser(DFSColourizer::new());
        let graph: SimpleGraph = G6Reader::read_graph(test_data::NO_SNARK_IN_G6_18).unwrap();
        let e_resistance = res_tester.edge_resistance(&graph);
        let v_resistance = res_tester.vertex_resistance(&graph);
        assert_eq!(e_resistance.is_some(), true);
        assert_eq!(e_resistance.unwrap(), 0);
        assert_eq!(v_resistance.is_some(), true);
        assert_eq!(v_resistance.unwrap(), 0);
    }

    #[test]
    fn should_have_resistance_two() {
        let res_tester = Resistance::new_with_colouriser(DFSColourizer::new());
        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_26_CRITICAL_1).unwrap();
        let e_resistance = res_tester.edge_resistance(&graph);
        let v_resistance = res_tester.vertex_resistance(&graph);
        assert_eq!(e_resistance.is_some(), true);
        assert_eq!(e_resistance.unwrap(), 2);
        assert_eq!(v_resistance.is_some(), true);
        assert_eq!(v_resistance.unwrap(), 2);
    }

    // too long test
    // #[test]
    // fn should_have_resistance_three() {
    //     let res_tester = Resistance::new_with_colourizer(SATColourizer::new());
    //     let graph: SimpleGraph =
    //         G6Reader::read_graph(test_data::SNARK_IN_G6_76).unwrap();
    //     let v_resistance = res_tester.vertex_resistance(&graph);
    //     assert_eq!(v_resistance.is_some(), true);
    //     assert_eq!(v_resistance.unwrap(), 3);
    // }

    #[test]
    fn should_have_resistance_three() {
        let res_tester = Resistance::new_with_colouriser(DFSColourizer::new());
        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_36_STABLE_RES_3).unwrap();
        let v_resistance = res_tester.vertex_resistance(&graph);
        assert_eq!(v_resistance.is_some(), true);
        assert_eq!(v_resistance.unwrap(), 3);
    }
}
