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
