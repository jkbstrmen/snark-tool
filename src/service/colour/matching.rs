use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::undirected::simple_graph::SimpleGraph;
use crate::graph::vertex::Vertex;
use crate::service::colour::colouriser::Colourizer;
use std::cmp;
use std::cmp::Ordering;
use std::ops::Deref;

pub struct MatchingColouriser {}

impl Colourizer for MatchingColouriser {
    fn is_colorable<G: Graph>(graph: &G) -> bool {

        println!("find all perfect matchings");
        let matchings = find_perfect_matchings(graph);

        println!("perfect matchings count: {}", matchings.len());

        println!("find max disjoint matchings");
        let disjoint_perfect_matchings = find_max_disjoing_matchings(&matchings);

        if disjoint_perfect_matchings.len() < 3 {
            return false;
        }

        true

        // let mut local_graph = SimpleGraph::from_graph(graph);
        //
        // for _i in 0..3 {
        //
        //     let blossom_graph = graph_to_blossom_graph(&local_graph);
        //
        //     let matching = blossom_graph.maximum_matching();
        //     let matching_edges = matching.edges();
        //     let mut matching_vertices = matching.vertices();
        //     let mut graph_vertices = blossom_graph.vertices().to_vec();
        //
        //     matching_vertices.sort();
        //     graph_vertices.sort();
        //     if !graph_vertices.eq(&matching_vertices) {
        //         return false;
        //     }
        //
        //     for matching_edge in matching_edges {
        //         local_graph.remove_edge(matching_edge.0, matching_edge.1);
        //     }
        // }
        // true
    }

    fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Matching {
    edges: Vec<UndirectedEdge>,
}

impl Matching {
    pub fn new() -> Self {
        Matching { edges: vec![] }
    }

    // add_edge

    pub fn sort_edges(&mut self) {
        self.edges.sort_by(|a, b| Self::edge_compare(a, b));
    }

    fn edge_compare(first: &UndirectedEdge, second: &UndirectedEdge) -> cmp::Ordering {
        let compare_from = first.from().cmp(&second.from());
        if cmp::Ordering::Equal.eq(&compare_from) {
            return first.to().cmp(&second.to());
        }
        compare_from
    }

    pub fn vertices(&self) -> Vec<usize> {
        let mut vertices = vec![];
        for edge in self.edges.iter() {
            vertices.push(edge.from());
            vertices.push(edge.to());
        }
        vertices
    }
}

pub fn find_perfect_matchings<G: Graph>(graph: &G) -> Vec<Matching> {
    let mut matchings = vec![];
    let local_graph = SimpleGraph::from_graph(graph);
    let matching = Matching::new();
    find_perfect_matchings_recursive(local_graph, matching, &mut matchings);
    matchings
}

fn find_perfect_matchings_recursive(
    graph: SimpleGraph,
    mut matching: Matching,
    matchings: &mut Vec<Matching>,
) {
    if graph.edges.is_empty() {
        if matching.vertices().len() == graph.size() {
            matching.sort_edges();
            matchings.push(matching);
            matchings.sort();
            matchings.dedup();
        }
        return;
    }
    for edge in graph.edges() {
        let mut local_graph = graph.clone();
        let mut local_matching = matching.clone();

        local_graph.remove_edges_of_vertex(edge.from());
        local_graph.remove_edges_of_vertex(edge.to());
        local_matching.edges.push(edge.clone());

        find_perfect_matchings_recursive(local_graph, local_matching, matchings);
    }
}

fn find_max_disjoing_matchings(matchings: &Vec<Matching>) -> Vec<Matching> {
    let mut max_disjoint = vec![];
    let mut index = 0;
    for _matching in matchings {

        let mut local_matchings = matchings.clone();
        let local_matching = local_matchings.remove(index);

        let mut disjoint_matchings = vec![];
        disjoint_matchings.push(local_matching);
        find_disjoing_matchings(&local_matchings, &mut disjoint_matchings);

        if max_disjoint.len() < disjoint_matchings.len() {
            max_disjoint = disjoint_matchings;
        }

        index += 1;
    }
    max_disjoint
}

fn find_disjoing_matchings(
    matchings: &Vec<Matching>,
    disjoint_matchings: &mut Vec<Matching>,
) {
    for matching in matchings {
        let mut disjoint = true;
        for disjoint_matching in disjoint_matchings.iter() {
            let mut matching_edges = matching.edges.clone();
            let mut disjoint_matching_edges = disjoint_matching.edges.clone();

            let mut edges: Vec<UndirectedEdge> = vec![];
            edges.append(&mut matching_edges);
            edges.append(&mut disjoint_matching_edges);

            edges.sort();
            let edges_count_before = edges.len();
            edges.dedup();
            if edges.len() < edges_count_before {
                disjoint = false;
                break;
            }
        }

        if disjoint {
            disjoint_matchings.push(matching.clone());
        }
    }
}

fn graph_to_blossom_graph<G: Graph>(graph: &G) -> blossom::Graph {
    // let graph: blossom::Graph::new();
    // let result_graph = blossom::graph::AnnotatedGraph::new();

    let mut blossom_graph = Vec::with_capacity(graph.size());

    for vertex in graph.vertices() {
        let mut neighbors = vec![];
        for edge in graph.edges_of_vertex(vertex.index()) {
            let neighbor = if edge.from() == vertex.index() {
                edge.to()
            } else {
                edge.from()
            };
            neighbors.push(neighbor);
        }
        if !neighbors.is_empty() {
            blossom_graph.push((vertex.index(), neighbors));
        }
    }

    let result: blossom::Graph = blossom_graph.iter().collect();
    result
}

// ===============================================

struct MatchingGraph {

}

impl MatchingGraph {

}

struct PerfectMatchings {

}

impl PerfectMatchings {

    fn generate_perfect_matchings(&self) {

        // if every connected component has an even number of vertices
        let mut odd_length = false;
        for connected_component in self.connected_components().iter() {
            if connected_component.len() % 2 == 1 {
                odd_length = true;
            }
        }
        if !odd_length {
            let next_vertex = 0; // next_vertex



        }


        // # if every connected component has an even number of vertices
        // if not any(len(cc) % 2 for cc in self.connected_components()):
        // v = next(self.vertex_iterator())
        // for e in self.edges_incident(v, labels=labels):
        //     Gp = self.copy(immutable=False)
        // Gp.delete_vertices([e[0], e[1]])
        // for mat in Gp.perfect_matchings(labels):
        // yield [e] + mat

    }

    fn connected_components(&self) -> Vec<Vec<usize>> {
        vec![]
    }

}