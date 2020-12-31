use crate::graph::edge::Edge;
use crate::graph::graph::{Graph, GraphConstructor};
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::vertex::Vertex;
use crate::service::component_analysis::edge_pairs::PairsOfNonAdjacentEdges;
use crate::service::component_analysis::vertex_pairs::PairsOfAdjacentVertices;

///
/// We need two graphs and two non adjacent edges of first graph and two adjacent
/// vertices of second graph
///
/// if graph_g and graph_h are snarks
/// if first_edge_of_g and second_edge_of_g are not adjacent
/// if first_vertex_of_h and second_vertex_of_h are adjacent
/// result graph will be snark as well
///
pub fn dot_product<G: Graph<E = UndirectedEdge> + Clone + GraphConstructor, V: Vertex>(
    graph_g: &G,
    graph_h: &G,
    edge_of_g_first: &UndirectedEdge,
    edge_of_g_second: &UndirectedEdge,
    vertex_of_h_first: &V,
    vertex_of_h_second: &V,
) -> G {
    let mut graph_gh = concat_graphs(graph_g, graph_h);
    let graph_h_begin_index = graph_g.size();

    // // take two non adjacent edges of G and remove them
    graph_gh.remove_edge(edge_of_g_first.from(), edge_of_g_first.to());
    graph_gh.remove_edge(edge_of_g_second.from(), edge_of_g_second.to());

    // take two adjacent vertices {x, y} of H and remove them along with edges of these vertices
    graph_gh.remove_edges_of_vertex(vertex_of_h_first.index() + graph_h_begin_index);
    graph_gh.remove_edges_of_vertex(vertex_of_h_second.index() + graph_h_begin_index);

    // resolve neighbors of two removed vertices from graph_h
    let mut neighbors_of_first_vertex = vec![];
    for neighbor in graph_h.neighbors_of_vertex(vertex_of_h_first.index()) {
        if neighbor != vertex_of_h_second.index() {
            neighbors_of_first_vertex.push(neighbor);
        }
    }
    let mut neighbors_of_second_vertex = vec![];
    for neighbor in graph_h.neighbors_of_vertex(vertex_of_h_second.index()) {
        if neighbor != vertex_of_h_first.index() {
            neighbors_of_second_vertex.push(neighbor);
        }
    }
    if neighbors_of_first_vertex.len() != 2 || neighbors_of_second_vertex.len() != 2 {
        // TODO - handle somehow - return Result
    }
    // connect each vertex of order 2 of graph G with vertex of order 2 of graph H
    graph_gh.add_edge(
        edge_of_g_first.from(),
        neighbors_of_first_vertex[0] + graph_h_begin_index,
    );
    graph_gh.add_edge(
        edge_of_g_first.to(),
        neighbors_of_first_vertex[1] + graph_h_begin_index,
    );
    graph_gh.add_edge(
        edge_of_g_second.from(),
        neighbors_of_second_vertex[0] + graph_h_begin_index,
    );
    graph_gh.add_edge(
        edge_of_g_second.to(),
        neighbors_of_second_vertex[1] + graph_h_begin_index,
    );

    wipe_vertices_without_edges(&graph_gh)
}

pub fn concat_graphs<G: Graph + Clone>(first: &G, second: &G) -> G {
    let mut result = first.clone();
    let second_begin_index = first.size();
    for edge in first.edges() {
        result.add_edge(edge.from(), edge.to());
    }
    for edge in second.edges() {
        result.add_edge(
            second_begin_index + edge.from(),
            second_begin_index + edge.to(),
        );
    }
    result
}

///
/// iterator over possible dot products of given two graphs
///
/// Be aware that graph isomorphism for result dot products is not checked and so result graph
/// could be the same as previous or next one
///
pub struct DotProducts<'a, G: Graph<E = UndirectedEdge>> {
    graph_g: &'a G,
    graph_h: &'a G,
    non_adjacent_edge_pairs_of_g: PairsOfNonAdjacentEdges<'a, G>,
    adjacent_vertex_pairs_of_h: PairsOfAdjacentVertices<'a, G::V, G>,
}

impl<'a, G: Graph<E = UndirectedEdge> + Clone + GraphConstructor> DotProducts<'a, G> {
    pub fn new(graph_g: &'a G, graph_h: &'a G) -> Self {
        DotProducts {
            graph_g,
            graph_h,
            non_adjacent_edge_pairs_of_g: PairsOfNonAdjacentEdges::new(graph_g),
            adjacent_vertex_pairs_of_h: PairsOfAdjacentVertices::new(graph_h),
        }
    }
}

impl<'a, G: Graph<E = UndirectedEdge> + Clone + GraphConstructor> Iterator for DotProducts<'a, G> {
    type Item = G;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(edges) = self.non_adjacent_edge_pairs_of_g.next() {
            if let Some(vertices) = self.adjacent_vertex_pairs_of_h.next() {
                let graph = dot_product(
                    self.graph_g,
                    self.graph_h,
                    edges.0,
                    edges.1,
                    vertices.0,
                    vertices.1,
                );
                return Some(graph);
            }
        }
        None
    }
}

///
/// reindex
///
fn wipe_vertices_without_edges<G: Graph + GraphConstructor>(graph: &G) -> G {
    let mut vertices_without_edges = vec![];
    for vertex in graph.vertices() {
        if graph.neighbors_of_vertex(vertex.index()).is_empty() {
            vertices_without_edges.push(vertex.index());
        }
    }
    let mut removed_vertex_iter = vertices_without_edges.iter();
    let mut removed_vertex_next = removed_vertex_iter.next();

    // create index mapping: old -> new
    let mut index_mapping = vec![0; graph.size()];
    let mut counter = 0;
    for vertex in graph.vertices() {
        if let Some(removed) = removed_vertex_next {
            if *removed == vertex.index() {
                counter += 1;
                removed_vertex_next = removed_vertex_iter.next();
                continue;
            }
        }
        index_mapping[vertex.index()] = vertex.index() - counter;
    }

    let mut final_graph = G::with_vertices_capacity(graph.size() - vertices_without_edges.len());
    for edge in graph.edges() {
        final_graph.add_edge(index_mapping[edge.from()], index_mapping[edge.to()]);
    }
    final_graph
}
