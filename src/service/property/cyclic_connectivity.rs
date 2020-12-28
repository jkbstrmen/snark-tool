use crate::graph::graph::{Graph, GraphConstructor};
use crate::graph::undirected_sparse::graph::SimpleSparseGraph;
use crate::graph::vertex::Vertex;
use crate::service::graph_traversal::bfs::BfsOfGraph;
use crate::service::property::girth::girth;
use crate::graph::multi::graph::MultiGraph;

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

                let max_flow = contract_full_trees_and_find_max_flow(graph, &ftv, &ftw);
                if max_flow < (3 * (2 as usize).pow(depth)) && max_flow < cut_size {
                    cut_size = max_flow;
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

pub fn full_tree<G: Graph>(graph: &G, root_vertex: usize, depth: u32) -> SimpleSparseGraph {
    let mut bfs = BfsOfGraph::new(graph, root_vertex);
    let mut full_tree = SimpleSparseGraph::new();

    while let Some(next) = bfs.next() {
        if next.distance_from_root() > depth as usize {
            break;
        }
        full_tree.add_edge(next.index(), next.discovered_from());
    }

    full_tree
}

pub fn vertex_disjoint_graphs(
    first_graph: &SimpleSparseGraph,
    second_graph: &SimpleSparseGraph,
) -> bool {
    for vertex in first_graph.vertices() {
        if first_graph.has_vertex(vertex.index()) && second_graph.has_vertex(vertex.index()) {
            return false;
        }
    }
    true
}

fn contract_full_trees_and_find_max_flow<G: Graph>(
    graph: &G,
    first_full_tree: &SimpleSparseGraph,
    second_full_tree: &SimpleSparseGraph,
) -> usize {
    let contracted_first = contract_sub_graph(graph, first_full_tree);
    let source = contracted_first.1;
    let contracted_second = contract_sub_graph(&contracted_first.0, second_full_tree);
    let sink = contracted_second.1;

    if source.is_none() || sink.is_none() {
        return 0;
    }

    max_flow(&contracted_second.0, source.unwrap(), sink.unwrap())
}

///
/// In given graph contract vertices of given sub-graph to one vertex
/// method returns graph with contracted sub-graph and index of vertex of output graph representing this contracted sub_graph
/// Given sub_graph should be one connected component of original graph.
///
pub fn contract_sub_graph<G: Graph>(
    graph: &G,
    sub_graph_to_contract: &SimpleSparseGraph,
) -> (MultiGraph, Option<usize>) {
    let output_graph = MultiGraph::from_graph(graph);
    let representing_vertex = sub_graph_to_contract.first_vertex();
    if representing_vertex.is_none() {
        return (output_graph, None);
    }
    let representing_vertex = representing_vertex.unwrap().index();

    // TODO - finish contracting
    // could be for all edges of sub_graph

    for vertex in sub_graph_to_contract.vertices() {
        if vertex.index() == representing_vertex {
            continue;
        }

        for neighbor in graph.neighbors_of_vertex(vertex.index()) {

        }

    }


    unimplemented!()
}

fn max_flow<G: Graph>(graph: &G, source: usize, sink: usize) -> usize {
    unimplemented!()
}
