use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::undirected::simple_graph::SimpleGraph;

///
/// if graph is snark and first_edge and second_edge are non adjacent and removable
/// -> output graph will be snark as well
///
pub fn i_extension<G: Graph+Clone, E: Edge>(graph: &G, first_edge: &E, second_edge: &E) -> G {
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

pub fn i_extension_arbitrary<G: Graph+Clone, E: Edge>(graph: &G, first_edge: &E, second_edge: &E) -> G {

    // resolve edges to apply i_extension to

    // apply i extension

    unimplemented!()
}