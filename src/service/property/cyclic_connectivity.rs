use crate::graph::graph::{Graph, GraphConstructor};
use crate::graph::undirected::multi_graph::graph::MultiGraph;
use crate::graph::undirected::simple_graph::graph::SimpleGraph;
use crate::graph::vertex::Vertex;
use crate::service::graph_traversal::bfs::BfsOfGraph;
use crate::service::property::girth::girth;
use crate::service::property::max_flow::max_flow::FordFulkerson;
use serde::export::Option::Some;

// TODO - needs optimizations

/**
 * Algorithm for finding cyclic edge connectivity of cubic simple graph with at
 * least 8 vertices
 * Based on An Algorithm for Cyclic Edge Connectivity of Cubic Graphs
 * work of Z. Dvorak, J. Kara, D. Kral and O. Pangrac
 * Department of Applied Mathematics and Institute for Theoretical Computer
 * Science Charles University
 *
*/
pub fn cyclic_edge_connectivity<G: Graph>(graph: &G) -> Option<usize> {
    let mut cut_size = girth(graph);

    for vertex_v in graph.vertices() {
        for vertex_w in graph.vertices() {
            if vertex_v.index() == vertex_w.index() {
                continue;
            }
            let mut depth = 0;
            loop {
                // Tv - treeVertex // a full tree of depth depth rooted at vertex_v
                let ftv = full_tree(graph, vertex_v.index(), depth);

                // Tw - treeVertex // a full tree of depth depth rooted at vertex_w
                let ftw = full_tree(graph, vertex_w.index(), depth);

                // if (Tv and Tw are not vertex-disjoint) break
                if !vertex_disjoint_graphs(&ftv, &ftw) {
                    break;
                }

                let paths_count = find_paths_count(graph, &ftv, &ftw);
                if paths_count < (3 * (2 as usize).pow(depth)) && paths_count < cut_size {
                    cut_size = paths_count;
                }
                if (3 * (2 as usize).pow(depth)) >= cut_size {
                    break;
                }
                depth += 1;
            }
        }
    }
    Some(cut_size)
}

pub fn full_tree<G: Graph>(graph: &G, root_vertex: usize, depth: u32) -> SimpleGraph {
    let mut bfs = BfsOfGraph::new(graph, root_vertex);
    let mut full_tree = SimpleGraph::new();
    full_tree.add_vertex_with_index(root_vertex);

    while let Some(next) = bfs.next() {
        if next.distance_from_root() > depth as usize {
            break;
        }
        full_tree.add_edge(next.index(), next.discovered_from());
    }

    full_tree
}

pub fn vertex_disjoint_graphs(first_graph: &SimpleGraph, second_graph: &SimpleGraph) -> bool {
    for vertex in first_graph.vertices() {
        if first_graph.has_vertex(vertex.index()) && second_graph.has_vertex(vertex.index()) {
            return false;
        }
    }
    true
}

///
/// In given graph contract vertices of given sub-graph to one vertex
/// method returns graph with contracted sub-graph and index of vertex of output graph representing this contracted sub_graph
/// Given sub_graph should be one connected component of original graph.
///
pub fn contract_sub_graph<G: Graph>(
    graph: &G,
    sub_graph_to_contract: &SimpleGraph,
) -> (MultiGraph, Option<usize>) {
    let mut output_graph = MultiGraph::from_graph(graph);
    let representing_vertex = sub_graph_to_contract.first_vertex();
    if representing_vertex.is_none() {
        return (output_graph, None);
    }
    let representing_vertex = representing_vertex.unwrap().index();

    for vertex in sub_graph_to_contract.vertices() {
        if vertex.index() == representing_vertex {
            continue;
        }
        for neighbor in graph.neighbors_of_vertex(vertex.index()) {
            if !sub_graph_to_contract.has_vertex(neighbor) {
                output_graph.add_edge(representing_vertex, neighbor);
            }
        }
        output_graph.remove_vertex(vertex.index());
    }
    (output_graph, Some(representing_vertex))
}

///
/// returns count of edge-disjoint paths from first full tree to second full tree
/// for now only count
///
fn find_paths_count<G: Graph>(
    graph: &G,
    first_full_tree: &SimpleGraph,
    second_full_tree: &SimpleGraph,
) -> usize {
    let contracted_first = contract_sub_graph(graph, first_full_tree);
    let source = contracted_first.1;
    let contracted_second = contract_sub_graph(&contracted_first.0, second_full_tree);
    let sink = contracted_second.1;

    if source.is_none() || sink.is_none() {
        return 0;
    }
    let source = source.unwrap();
    let sink = sink.unwrap();
    let contracted_graph = contracted_second.0;

    FordFulkerson::max_flow(&contracted_graph, source, sink)
}
