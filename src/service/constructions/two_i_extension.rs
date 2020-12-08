use crate::graph::edge::Edge;
use crate::graph::graph::Graph;

///
/// if graph is snark and first, second and third edge are non adjacent?? and removable
/// -> output graph will be snark as well
///
pub fn two_i_extension<G: Graph + Clone, E: Edge>(
    graph: &G,
    first_edge: &E,
    second_edge: &E,
    third_edge: &E,
) -> G {
    // let mut result_graph = (*graph).clone();
    // result_graph.remove_edge(first_edge.from(), first_edge.to());
    // result_graph.remove_edge(second_edge.from(), second_edge.to());
    // result_graph.remove_edge(third_edge.from(), third_edge.to());
    //
    // let first_new_vertex = graph.size();
    // let second_new_vertex = graph.size() + 1;
    // let third_new_vertex = graph.size() + 2;
    // let fourth_new_vertex = graph.size() + 3;
    // result_graph.add_edge(first_edge.from(), first_new_vertex);
    // result_graph.add_edge(first_edge.to(), first_new_vertex);
    // result_graph.add_edge(second_edge.from(), second_new_vertex);
    // result_graph.add_edge(second_edge.to(), second_new_vertex);
    // result_graph.add_edge(third_edge.from(), third_new_vertex);
    // result_graph.add_edge(third_edge.to(), third_new_vertex);
    //
    // result_graph.add_edge(first_new_vertex, fourth_new_vertex);
    // result_graph.add_edge(second_new_vertex, fourth_new_vertex);
    // result_graph.add_edge(third_new_vertex, fourth_new_vertex);
    // result_graph

    unimplemented!()
}

pub fn two_i_extension_arbitrary<G: Graph + Clone, E: Edge>(graph: &G) -> G {
    // resolve edges to apply two i extension to

    // apply two i extension

    unimplemented!()
}
