use crate::graph::edge::{Edge, EdgeConstructor};
use crate::graph::graph::Graph;
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::undirected::simple_graph::graph::SimpleGraph;
use crate::service::matching::perfect_matchings::{Matching, MatchingGraph};
use serde::export::Option::Some;
use std::cmp::Ordering;
use std::iter::FromIterator;
use std::ops::Add;

struct MatchingPair<'a> {
    first: &'a Matching,
    second: &'a Matching,
    common_edges: Vec<UndirectedEdge>,
}

impl<'a> MatchingPair<'a> {
    pub fn new(
        first: &'a Matching,
        second: &'a Matching,
        common_edges: Vec<UndirectedEdge>,
    ) -> Self {
        MatchingPair {
            first,
            second,
            common_edges,
        }
    }
}

struct MatchingsCombination<'a> {
    // pair (matching, index of matching in original vector)
    // matchings: Vec<(Matching, usize)>,

    // pair (matching, true if used in combination)
    matchings: Vec<(&'a Matching, bool)>,
    non_covered_edges: Vec<UndirectedEdge>,
    common_edges: Vec<UndirectedEdge>,
    used_matchings: Vec<usize>,
}

impl<'a> MatchingsCombination<'a> {
    pub fn new(
        matchings: Vec<(&'a Matching, bool)>,
        non_covered_edges: Vec<UndirectedEdge>,
        common_edges: Vec<UndirectedEdge>,
        used_matchins: Vec<usize>,
    ) -> Self {
        MatchingsCombination {
            matchings,
            non_covered_edges,
            common_edges,
            used_matchings: used_matchins,
        }
    }
}

// struct MatchingPair<'a>{
//     first: &Matching,
//     second: &Matching,
//     similarity: usize
// }

pub struct PerfectMatchingIndex<'a> {
    matchings: Vec<(Matching, bool)>,

    // pairs: Vec<MatchingPair>
    combinations: Vec<MatchingsCombination<'a>>,
}

///
/// Works only for cubic graphs
///
pub fn perfect_matching_index<G: Graph>(graph: &G) -> usize {
    // check if graph is colorable - if it is - pmi is 3

    // if it isn't - pmi could be 4, 5 or maybe 6

    // let mut pmi = PerfectMatchingIndex {
    //     matchings: vec![],
    //     // pairs: vec![],
    //     combinations: vec![],
    // };
    // for perfect_matching in perfect_matchings {
    //     pmi.matchings.push((perfect_matching, false));
    // }
    // let mut graph_copy = SimpleGraph::from_graph(graph);

    // create all possible pairs of matchings
    // computy similarity index - number of common edges - sort ascending
    // if first in vec has similarity index 0 - pmi is 3

    // foreach pair and each unused matching
    // compare to whole edge set - find not covered edge(s)
    // try to find unused matching containing this edge(s)
    // if not successful what?
    // go to next pair
    // when checked all pairs - pmi is higher than 4?
    //
    // store all pairs,
    //  then all triples
    //  then all foursomes
    //  then all fivesomes
    //  then compute for all possible fivesomes - find unused matching containing non covered edge

    // at any time we would know minimum number of non covered edges

    // if would be enough
    // let edge_set = Vec::from_iter(graph.edges());

    let mut edge_set = vec![];
    for edge in graph.edges() {
        edge_set.push(UndirectedEdge::new(edge.from(), edge.to()));
    }
    edge_set.sort();

    let mut matching_graph = MatchingGraph::from_graph(graph);
    let mut perfect_matchings = matching_graph.perfect_matchings();
    for mut perfect_matching in perfect_matchings.iter_mut() {
        perfect_matching.edges.sort();
    }
    perfect_matchings.sort();

    let mut matchings_ref = vec![];
    for perfect_matching in perfect_matchings.iter() {
        matchings_ref.push((perfect_matching, false));
    }

    let mut pairs = vec![];
    // at first compute pair of matchings with similarity index - sort vec of pairs by it
    let mut first_counter = 0;
    let mut second_counter = 0;
    for first in perfect_matchings.iter() {
        for second in perfect_matchings.iter() {
            let common_edges = common_edges(&first.edges, &second.edges);
            // pairs.push(MatchingPair::new(first, second, common_edges));
            let mut matchings_ref_copy = matchings_ref.clone();
            matchings_ref_copy[first_counter].1 = true;
            matchings_ref_copy[second_counter].1 = true;

            pairs.push(MatchingsCombination::new(
                matchings_ref_copy,
                vec![],
                common_edges,
                vec![first_counter, second_counter],
            ));

            second_counter += 1;
        }
        second_counter = 0;
        first_counter += 1;
    }
    // sort pairs
    pairs.sort_by(|first, second| {
        let first_len = first.common_edges.len();
        let second_len = second.common_edges.len();
        if first_len < second_len {
            return Ordering::Less;
        }
        if first_len > second_len {
            return Ordering::Greater;
        }
        Ordering::Equal
    });
    if pairs[0].common_edges.len() == 0 {
        return 3;
    }
    // for mut pair in pairs.iter_mut() { // mut makes troubles with borrow checker
    for mut pair in pairs.iter() {
        let mut merged_matching_edges = vec![];
        for used_matching in pair.used_matchings.iter() {
            let matching: &Matching = pair.matchings[*used_matching].0;
            for edge in matching.edges.iter() {
                merged_matching_edges.push(edge.clone());
            }
        }
        // merged_matching_edges.sort();

        let mut counter = 0;
        for third_matching in pair.matchings.iter() {
            if !third_matching.1 {
                // TODO - at first choose only matchings containing common edges?

                // pair.matchings[counter].1 = true;

                // add unused matching to merged edges, then sort
                for edge in third_matching.0.edges.iter() {
                    merged_matching_edges.push(edge.clone());
                }
                merged_matching_edges.sort();

                let distinct_edges = distinct_edges(&edge_set, &merged_matching_edges);

                // for non covered edges - try to find fourth unused matching containing all these edges
                for fourth_matching in pair.matchings.iter() {
                    let common_edges = common_edges(&fourth_matching.0.edges, &distinct_edges);
                    if common_edges.len() == distinct_edges.len() {
                        return 4;
                    }
                }
            }
            counter += 1;
        }
    }

    // TODO - what if we got here?

    7

    // for max_nesting in 3..graph.size() {
    //     let mut temp = "".to_string();
    //
    //     // println!("{}", max_nesting);
    //
    //     let option = pmi.pmi_recursive(&mut graph_copy, max_nesting, &mut temp);
    //     if let Some(result) = option {
    //         return result;
    //     }
    // }
    // graph.size()
}

///
/// matchings has to be sorted
///
pub fn compute_matching_similarity(first: &Matching, second: &Matching) -> usize {
    let mut similarity = 0;
    if first.edges.len() == 0 {
        return 0;
    }

    let mut first_iter = first.edges.iter();
    let mut second_iter = second.edges.iter();

    let mut edge_first_next = first_iter.next();
    let mut edge_second_next = second_iter.next();

    loop {
        if let Some(edge_first) = edge_first_next {
            if let Some(edge_second) = edge_second_next {
                if edge_first == edge_second {
                    similarity += 1;
                }

                if edge_first.from() <= edge_second.from() {
                    edge_first_next = first_iter.next();
                }
                if edge_second.from() <= edge_first.from() {
                    edge_second_next = second_iter.next();
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    similarity
}

///
/// input matchings has to be sorted
///
fn common_edges(first: &Vec<UndirectedEdge>, second: &Vec<UndirectedEdge>) -> Vec<UndirectedEdge> {
    let mut common_edges = vec![];
    if first.len() == 0 {
        return vec![];
    }

    let mut first_iter = first.iter();
    let mut second_iter = second.iter();

    let mut edge_first_next = first_iter.next();
    let mut edge_second_next = second_iter.next();

    loop {
        if let Some(edge_first) = edge_first_next {
            if let Some(edge_second) = edge_second_next {
                if edge_first == edge_second {
                    common_edges.push(edge_first.clone());
                }

                if edge_first.from() <= edge_second.from() {
                    edge_first_next = first_iter.next();
                }
                if edge_second.from() <= edge_first.from() {
                    edge_second_next = second_iter.next();
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    common_edges
}

///
/// first matching can be of bigger size, but both has to be sorted
/// second has to be
///
pub fn distinct_edges(
    first: &Vec<UndirectedEdge>,
    second: &Vec<UndirectedEdge>,
) -> Vec<UndirectedEdge> {
    let mut distinct_edges = vec![];
    let mut first_iter = first.iter();
    let mut second_iter = second.iter();

    let mut edge_first_next = first_iter.next();
    let mut edge_second_next = second_iter.next();

    loop {
        if let Some(edge_first) = edge_first_next {
            if let Some(edge_second) = edge_second_next {
                if edge_first != edge_second {
                    distinct_edges.push(edge_first.clone());
                }

                if edge_first.from() <= edge_second.from() {
                    edge_first_next = first_iter.next();
                }
                if edge_second.from() <= edge_first.from() {
                    edge_second_next = second_iter.next();
                }
            } else {
                distinct_edges.push(edge_first.clone());
                while let Some(edge_first) = first_iter.next() {
                    distinct_edges.push(edge_first.clone());
                }
                break;
            }
        } else {
            break;
        }
    }
    distinct_edges
}

impl<'a> PerfectMatchingIndex<'a> {
    fn pmi_recursive(
        &mut self,
        graph: &mut SimpleGraph,
        max_nesting: usize,
        mut temp: &mut String,
    ) -> Option<usize> {
        // println!("{}", temp);

        if max_nesting == 0 {
            let mut has_edges = false;
            for _edge in graph.edges() {
                has_edges = true;
                break;
            }
            if !has_edges {
                return Some(0);
            }
            return None;
        }
        for i in 0..self.matchings.len() {
            let matching = &mut self.matchings[i];
            if matching.1 {
                continue;
            }
            matching.1 = true;
            for edge in matching.0.edges.iter() {
                graph.remove_edge(edge.from(), edge.to());
            }

            let mut temp_2 = (temp.clone()).add(format!(" {}", i).as_str());
            let local_index = self.pmi_recursive(graph, max_nesting - 1, &mut temp_2);

            if let Some(local) = local_index {
                // let matching = &mut self.matchings[i];
                // for edge in matching.0.edges.iter() {
                //     graph.add_edge(edge.from(), edge.to());
                // }
                // matching.1 = false;
                return Some(local + 1);
            }

            // reverse
            let matching = &mut self.matchings[i];
            for edge in matching.0.edges.iter() {
                graph.add_edge(edge.from(), edge.to());
            }
            matching.1 = false;
            // i += 1;
        }
        None
    }
}

// second try
// -------------------------------------------------------------------------------
pub fn perfect_matching_index_2<G: Graph>(graph: &G) -> usize {
    let mut matching_graph = MatchingGraph::from_graph(graph);
    let perfect_matchings = matching_graph.perfect_matchings();
    let mut pmi = PerfectMatchingIndex2 { matchings: vec![] };
    for perfect_matching in perfect_matchings {
        pmi.matchings.push((perfect_matching, false));
    }
    let mut graph_copy = SimpleGraph::from_graph(graph);

    for max_nesting in 3..graph.size() {
        let mut temp = "".to_string();

        // println!("{}", max_nesting);

        let option = pmi.pmi_recursive(&mut graph_copy, max_nesting, &mut temp);
        if let Some(result) = option {
            return result;
        }
    }
    graph.size()
}

pub struct PerfectMatchingIndex2 {
    matchings: Vec<(Matching, bool)>,
}

impl PerfectMatchingIndex2 {
    fn pmi_recursive(
        &mut self,
        graph: &mut SimpleGraph,
        max_nesting: usize,
        mut temp: &mut String,
    ) -> Option<usize> {
        // println!("{}", temp);

        if max_nesting == 0 {
            let mut has_edges = false;
            for _edge in graph.edges() {
                has_edges = true;
                break;
            }
            if !has_edges {
                return Some(0);
            }
            return None;
        }
        for i in 0..self.matchings.len() {
            let matching = &mut self.matchings[i];
            if matching.1 {
                continue;
            }
            matching.1 = true;
            for edge in matching.0.edges.iter() {
                graph.remove_edge(edge.from(), edge.to());
            }

            let mut temp_2 = (temp.clone()).add(format!(" {}", i).as_str());
            let local_index = self.pmi_recursive(graph, max_nesting - 1, &mut temp_2);

            if let Some(local) = local_index {
                // let matching = &mut self.matchings[i];
                // for edge in matching.0.edges.iter() {
                //     graph.add_edge(edge.from(), edge.to());
                // }
                // matching.1 = false;
                return Some(local + 1);
            }

            // reverse
            let matching = &mut self.matchings[i];
            for edge in matching.0.edges.iter() {
                graph.add_edge(edge.from(), edge.to());
            }
            matching.1 = false;
            // i += 1;
        }
        None
    }
}

// first try
//  -----------------------------------------------
pub fn perfect_matching_index_1<G: Graph>(graph: &G) -> usize {
    let mut matching_graph = MatchingGraph::from_graph(graph);
    let perfect_matchings = matching_graph.perfect_matchings();
    let mut pmi = PerfectMatchingIndex1 { matchings: vec![] };
    for perfect_matching in perfect_matchings {
        pmi.matchings.push((perfect_matching, false));
    }
    let mut graph_copy = SimpleGraph::from_graph(graph);
    let mut index = usize::max_value();

    for i in 0..pmi.matchings.len() {
        let matching = &mut pmi.matchings[i];
        matching.1 = true;
        for edge in matching.0.edges.iter() {
            graph_copy.remove_edge(edge.from(), edge.to());
        }

        let local_index = pmi.pmi_recursive(&mut graph_copy);
        if let Some(local) = local_index {
            if local < index {
                index = local;
            }
        }

        let matching = &mut pmi.matchings[i];
        for edge in matching.0.edges.iter() {
            graph_copy.add_edge(edge.from(), edge.to());
        }
        pmi.matchings[i].1 = false;
    }
    index + 1
}

pub struct PerfectMatchingIndex1 {
    matchings: Vec<(Matching, bool)>,
}

impl PerfectMatchingIndex1 {
    fn pmi_recursive(&mut self, graph: &mut SimpleGraph) -> Option<usize> {
        let mut has_edges = false;
        for _edge in graph.edges() {
            has_edges = true;
        }
        if !has_edges {
            return Some(0);
        }

        let mut index = usize::max_value();
        // let mut i = 0;
        // for matching in self.matchings.iter() {
        for i in 0..self.matchings.len() {
            let matching = &mut self.matchings[i];

            if matching.1 {
                // i += 1;
                continue;
            }
            matching.1 = true;
            for edge in matching.0.edges.iter() {
                graph.remove_edge(edge.from(), edge.to());
            }

            let local_index = self.pmi_recursive(graph);
            if let Some(local) = local_index {
                if local == 0 {
                    // reverse
                    let matching = &mut self.matchings[i];
                    for edge in matching.0.edges.iter() {
                        graph.add_edge(edge.from(), edge.to());
                    }
                    matching.1 = false;
                    return Some(1);
                }
                if local < index {
                    index = local;
                }
            }

            // reverse
            let matching = &mut self.matchings[i];
            for edge in matching.0.edges.iter() {
                graph.add_edge(edge.from(), edge.to());
            }
            matching.1 = false;
            // i += 1;
        }
        Some(index + 1)
    }
}
