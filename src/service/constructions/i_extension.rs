use crate::graph::edge::{Edge, EdgeConstructor};
use crate::graph::graph::Graph;
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::undirected::simple_graph::SimpleGraph;
use crate::graph::undirected_sparse::graph::Edges;
use crate::service::colour::colouriser::Colouriser;
use crate::service::component_analysis::edge_pairs::RemovablePairsOfEdges;
use serde::export::Option::Some;

///
/// I-extension will extend graph by two vertices
/// if graph is snark and first_edge and second_edge are non adjacent and removable
/// -> output graph will be snark as well
///
pub fn i_extension<G: Graph + Clone, E: Edge>(graph: &G, first_edge: &E, second_edge: &E) -> G {
    let mut result_graph = (*graph).clone();
    result_graph.remove_edge(first_edge.from(), first_edge.to());
    result_graph.remove_edge(second_edge.from(), second_edge.to());

    let first_new_vertex = graph.size();
    let second_new_vertex = graph.size() + 1;
    result_graph.add_edge(first_edge.from(), first_new_vertex);
    result_graph.add_edge(first_edge.to(), first_new_vertex);
    result_graph.add_edge(second_edge.from(), second_new_vertex);
    result_graph.add_edge(second_edge.to(), second_new_vertex);
    result_graph.add_edge(first_new_vertex, second_new_vertex);
    result_graph
}

pub struct IExtensions<'a, G: Graph + Clone, C: Colouriser> {
    graph: &'a G,
    colouriser: &'a C,
    removable_edge_pairs: RemovablePairsOfEdges<'a, G, C>,
}

impl<'a, G: Graph<E = UndirectedEdge> + Clone, C: Colouriser> Iterator for IExtensions<'a, G, C> {
    type Item = G;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pair) = self.removable_edge_pairs.next() {
            let graph = i_extension(self.graph, pair.0, pair.1);
            return Some(graph);
        }
        None
    }
}

impl<'a, G: Graph<E = UndirectedEdge> + Clone, C: Colouriser> IExtensions<'a, G, C> {
    pub fn new(graph: &'a G, colouriser: &'a C) -> Self {
        IExtensions {
            graph,
            colouriser,
            removable_edge_pairs: RemovablePairsOfEdges::new(&graph, &colouriser),
        }
    }
}
