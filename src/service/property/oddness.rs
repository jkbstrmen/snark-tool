use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::service::colour::matchings::matching_col::CycleDiscovery;
use crate::service::matching::perfect_matchings::MatchingGraph;

pub struct Oddness {}

impl Oddness {
    pub fn of_graph<G: Graph>(graph: &G) -> usize {
        let mut match_graph = MatchingGraph::from_graph(graph);
        let perfect_matchings = match_graph.perfect_matchings();
        let mut oddness = usize::max_value();
        for perfect_matching in perfect_matchings {
            // remove matching from graph
            for edge in perfect_matching.edges.iter() {
                match_graph.remove_edge(edge.from(), edge.to());
            }

            let odd_cycles_num = Self::num_of_odd_cycles(&match_graph);
            if odd_cycles_num < oddness {
                oddness = odd_cycles_num;
            }

            // recover removed edges
            for edge in perfect_matching.edges.iter() {
                match_graph.add_edge(edge.from(), edge.to());
            }
        }
        oddness
    }

    fn num_of_odd_cycles(graph: &MatchingGraph) -> usize {
        let mut odd_cycles_num = 0;
        let mut cd = CycleDiscovery::new(graph);
        for cycle in cd.cycles() {
            if cycle.len() % 2 == 1 {
                odd_cycles_num += 1;
            }
        }
        odd_cycles_num
    }
}

///
/// TESTS
///
#[cfg(test)]
mod tests {
    use crate::graph::undirected::simple_graph::graph::SimpleGraph;
    use crate::service::io::reader_g6::G6Reader;
    use crate::service::property::oddness::Oddness;
    use crate::tests::test_data::test_data;

    #[test]
    fn should_have_oddness_zero() {
        let graph: SimpleGraph = G6Reader::read_graph(test_data::NO_SNARK_IN_G6_18).unwrap();
        let oddness = Oddness::of_graph(&graph);
        assert_eq!(oddness, 0);
    }

    #[test]
    fn should_have_oddness_two() {
        let graph = test_data::get_petersen_graph();
        let oddness = Oddness::of_graph(&graph);
        assert_eq!(oddness, 2);

        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_26_CRITICAL_1).unwrap();
        let oddness = Oddness::of_graph(&graph);
        assert_eq!(oddness, 2);
    }

    #[test]
    fn should_have_oddness_four() {
        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_36_STABLE_RES_3).unwrap();
        let oddness = Oddness::of_graph(&graph);
        assert_eq!(oddness, 4);
    }
}
