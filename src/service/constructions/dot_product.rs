use crate::graph::edge::Edge;
use crate::graph::graph::{Graph, GraphConstructor};
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::undirected_sparse::graph::SimpleSparseGraph;
use crate::graph::undirected_sparse::vertex::VertexWithEdges;
use crate::graph::vertex::Vertex;

///
/// if graph_g and graph_h are snarks
/// if first_edge_of_g and second_edge_of_g are not adjacent
/// if first_vertex_of_h and second_vertex_of_h are adjacent
/// result graph will be snark as well
///
fn dot_product(
    graph_g: &SimpleSparseGraph,
    graph_h: &SimpleSparseGraph,
    first_edge_of_g: &UndirectedEdge,
    second_edge_of_g: &UndirectedEdge,
    first_vertex_of_h: &VertexWithEdges,
    second_vertex_of_h: &VertexWithEdges,
) -> SimpleSparseGraph {
    unimplemented!()
}

fn dot_product_arbitrary(
    graph_g: &SimpleSparseGraph,
    graph_h: &SimpleSparseGraph,
) -> SimpleSparseGraph {
    unimplemented!()
}

// TODO - refactor and use with Graph trait
pub fn dot_product_first(
    graph_g: &SimpleSparseGraph,
    graph_h: &SimpleSparseGraph,
    // first_edge_of_g: &UndirectedEdge,
    // second_edge_of_g: &UndirectedEdge,
    // first_vertex_of_h: &SimpleVertex,
    // second_vertex_of_h: &SimpleVertex,
) -> SimpleSparseGraph {
    // add graph_h to graph_g
    let mut graph_gh = graph_g.clone();
    let graph_h_begin_index = graph_g.size();
    for vertex in 0..graph_h.size() {
        graph_gh.add_vertex();
    }
    for edge in graph_h.edges() {
        graph_gh.add_edge(
            graph_h_begin_index + edge.from(),
            graph_h_begin_index + edge.to(),
        );
    }

    // take two non adjacent edges of G and remove them
    let mut first_edge: Option<&UndirectedEdge> = None;
    for edge in graph_g.edges() {
        first_edge = Some(edge);
        break;
    }

    if first_edge.is_none() {
        // TODO - handle
        panic!("first edge is none");
    }
    let first_edge = first_edge.unwrap();

    // find second edge - non adjacent to first
    let mut second_edge: Option<&UndirectedEdge> = None;
    for edge in graph_g.edges() {
        if !edge.is_adjacent(&first_edge) {
            second_edge = Some(edge);
            break;
        }
    }
    if second_edge.is_none() {
        // TODO - handle
        panic!("second edge is none");
    }
    let second_edge = second_edge.unwrap();
    graph_gh.remove_edge(first_edge.from(), first_edge.to());
    graph_gh.remove_edge(second_edge.from(), second_edge.to());

    // take two adjacent vertices {x, y} of H and remove them along with edges of these vertices
    let first_neighbor_of_x;
    let second_neighbor_of_x;
    let mut first_neighbor_of_y = 0;
    let mut second_neighbor_of_y = 0;

    let mut vertex_x: Option<&VertexWithEdges> = None;
    for vertex in graph_h.vertices() {
        vertex_x = Some(vertex);
        break;
    }
    if vertex_x.is_none() {
        // TODO - handle
        panic!("second edge is none");
    }

    let vertex_x = vertex_x.unwrap();
    let y_index = *(vertex_x.neighbors().get(0).unwrap());
    let vertex_y = &graph_h.vertices[*(vertex_x.neighbors().get(0).unwrap())].index();
    first_neighbor_of_x = graph_h.vertices[*(vertex_x.neighbors().get(1).unwrap())]
        .index()
        .clone();
    second_neighbor_of_x = graph_h.vertices[*(vertex_x.neighbors().get(2).unwrap())]
        .index()
        .clone();

    let vertex_x = vertex_x.index();
    let mut neighbors_of_y = graph_h.vertices[*vertex_y].neighbors().clone();
    neighbors_of_y.retain(|vertex| *vertex != vertex_x);
    first_neighbor_of_y = neighbors_of_y[0];
    second_neighbor_of_y = neighbors_of_y[1];

    graph_gh.remove_edges_of_vertex(vertex_x + graph_h_begin_index);
    graph_gh.remove_edges_of_vertex(vertex_y + graph_h_begin_index);

    // connect each vertex of order 2 of graph G with vertex of order 2 of graph H
    graph_gh.add_edge(first_edge.from(), first_neighbor_of_x + graph_h_begin_index);
    graph_gh.add_edge(first_edge.to(), second_neighbor_of_x + graph_h_begin_index);
    graph_gh.add_edge(
        second_edge.from(),
        first_neighbor_of_y + graph_h_begin_index,
    );
    graph_gh.add_edge(second_edge.to(), second_neighbor_of_y + graph_h_begin_index);

    // println!("{:?}", first_edge);
    // println!("{:?}", second_edge);
    // println!("{}", vertex_x + graph_h_begin_index);
    // println!("{}", vertex_y + graph_h_begin_index);

    wipe_vertices_without_edges(&graph_gh)
}

///
/// reindex
///
fn wipe_vertices_without_edges(graph: &SimpleSparseGraph) -> SimpleSparseGraph {
    let mut vertices_without_edges = vec![];
    for vertex in graph.vertices() {
        if vertex.neighbors().is_empty() {
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

    let mut final_graph =
        SimpleSparseGraph::with_vertices_capacity(graph.size() - vertices_without_edges.len());
    for edge in graph.edges() {
        final_graph.add_edge(index_mapping[edge.from()], index_mapping[edge.to()]);
    }
    final_graph
}
