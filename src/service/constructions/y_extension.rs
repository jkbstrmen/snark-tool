use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::service::colour::colouriser::Colouriser;
use crate::service::component_analysis::edge_triplets::RemovableTripletsOfEdges;
use crate::graph::undirected::edge::UndirectedEdge;

///
/// if graph is snark and first, second and third edge are removable
/// -> output graph will be snark as well
///
pub fn y_extension<G: Graph + Clone, E: Edge>(
    graph: &G,
    first_edge: &E,
    second_edge: &E,
    third_edge: &E,
) -> G {
    let mut result_graph = (*graph).clone();
    result_graph.remove_edge(first_edge.from(), first_edge.to());
    result_graph.remove_edge(second_edge.from(), second_edge.to());
    result_graph.remove_edge(third_edge.from(), third_edge.to());

    let first_new_vertex = graph.size();
    let second_new_vertex = graph.size() + 1;
    let third_new_vertex = graph.size() + 2;
    let fourth_new_vertex = graph.size() + 3;
    result_graph.add_edge(first_edge.from(), first_new_vertex);
    result_graph.add_edge(first_edge.to(), first_new_vertex);
    result_graph.add_edge(second_edge.from(), second_new_vertex);
    result_graph.add_edge(second_edge.to(), second_new_vertex);
    result_graph.add_edge(third_edge.from(), third_new_vertex);
    result_graph.add_edge(third_edge.to(), third_new_vertex);

    result_graph.add_edge(first_new_vertex, fourth_new_vertex);
    result_graph.add_edge(second_new_vertex, fourth_new_vertex);
    result_graph.add_edge(third_new_vertex, fourth_new_vertex);
    result_graph
}

pub struct YExtensions<'a, G: Graph + Clone, C: Colouriser> {
    graph: &'a G,
    colouriser: &'a C,
    removable_edge_triplets: RemovableTripletsOfEdges<'a, G, C>,
}

impl<'a, G: Graph<E = UndirectedEdge> + Clone, C: Colouriser> Iterator for YExtensions<'a, G, C> {
    type Item = G;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(triplet) = self.removable_edge_triplets.next() {
            let graph = y_extension(self.graph, triplet.0, triplet.1, triplet.2);
            return Some(graph);
        }
        None
    }
}

impl<'a, G: Graph<E = UndirectedEdge> + Clone, C: Colouriser> YExtensions<'a, G, C> {
    pub fn new(graph: &'a G, colouriser: &'a C) -> Self {
        YExtensions {
            graph,
            colouriser,
            removable_edge_triplets: RemovableTripletsOfEdges::new(&graph, &colouriser),
        }
    }
}
