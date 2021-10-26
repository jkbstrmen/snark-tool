use crate::graph::graph::{Graph, GraphConstructor};
use crate::graph::undirected::multi_graph::graph::MultiGraph;
use crate::graph::undirected::simple_graph::graph::SimpleGraph;
use crate::graph::vertex::Vertex;
use crate::service::graph_traversal::bfs::BfsOfGraph;
use crate::service::property::girth::girth;
use crate::service::property::max_flow::max_flow::FordFulkerson;

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

///
/// TESTS
///
#[cfg(test)]
mod tests {
    use crate::graph::graph::{Graph, GraphConstructor};
    use crate::graph::undirected::simple_graph::graph::SimpleGraph;
    use crate::graph::vertex::Vertex;
    use crate::service::io::reader::GraphFileReader;
    use crate::service::io::reader_g6::G6Reader;
    use crate::service::property::cyclic_connectivity::{
        contract_sub_graph, cyclic_edge_connectivity, full_tree, vertex_disjoint_graphs,
    };
    use crate::tests::test_data::test_data;
    use std::fs;

    #[test]
    fn should_have_cec_five() {
        let graph = test_data::get_petersen_graph();
        let cec = cyclic_edge_connectivity(&graph);
        assert_eq!(cec.is_some(), true);
        assert_eq!(cec.unwrap(), 5);

        let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_30_CYC_5).unwrap();
        let cec = cyclic_edge_connectivity(&graph);
        assert_eq!(cec.is_some(), true);
        assert_eq!(cec.unwrap(), 5);
    }

    #[test]
    fn should_have_cec_three() {
        let graph = test_data::get_falcon_graph();
        let cec = cyclic_edge_connectivity(&graph);
        assert_eq!(cec.is_some(), true);
        assert_eq!(cec.unwrap(), 3);
    }

    #[test]
    fn should_have_cec_four_file() {
        let path = test_data::GG_30_G05_CYC4_G6_100_FILE_PATH;
        let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();
        let reader = G6Reader::<SimpleGraph>::new(&file_result);

        test_graphs_for_cec(reader, 4);
    }

    #[test]
    fn should_have_cec_five_file() {
        let path = test_data::GG_30_G05_CYC5_G6_100_FILE_PATH;
        let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();
        let reader = G6Reader::<SimpleGraph>::new(&file_result);

        test_graphs_for_cec(reader, 5);
    }

    fn test_graphs_for_cec<G: Graph + GraphConstructor>(
        mut reader: G6Reader<G>,
        cyclic_edge_conn: usize,
    ) {
        while let Some(graph_result) = reader.next() {
            let graph = graph_result.unwrap();
            let cec = cyclic_edge_connectivity(&graph);
            assert_eq!(cec.is_some(), true);
            assert_eq!(cec.unwrap(), cyclic_edge_conn);
        }
    }

    #[test]
    fn should_create_full_tree() {
        let graph = test_data::get_falcon_graph();

        let ft = full_tree(&graph, 26, 1);
        assert_eq!(ft.has_vertex(26), true);
        assert_eq!(ft.has_vertex(20), true);
        assert_eq!(ft.has_vertex(21), true);
        assert_eq!(ft.has_vertex(27), true);
        for i in 0..19 {
            assert_eq!(ft.has_vertex(i), false)
        }
        assert_eq!(ft.has_vertex(22), false);
        assert_eq!(ft.has_vertex(23), false);
        assert_eq!(ft.has_vertex(24), false);
        assert_eq!(ft.has_vertex(25), false);
        assert_eq!(ft.size(), 28);
        assert_eq!(ft.first_vertex().unwrap().index(), 20);
        let vertices_check = vec![20, 21, 26, 27];
        let mut vertices = vec![];
        for vertex in ft.vertices() {
            vertices.push(vertex.index());
        }
        assert_eq!(vertices, vertices_check);

        let ft = full_tree(&graph, 5, 2);
        assert_eq!(ft.has_vertex(5), true);
        assert_eq!(ft.has_vertex(3), true);
        assert_eq!(ft.has_vertex(4), true);
        assert_eq!(ft.has_vertex(31), true);
        assert_eq!(ft.has_vertex(0), true);
        assert_eq!(ft.has_vertex(8), true);
        assert_eq!(ft.has_vertex(2), true);
        assert_eq!(ft.has_vertex(7), true);
        assert_eq!(ft.has_vertex(30), true);
        assert_eq!(ft.has_vertex(28), true);

        assert_eq!(ft.has_vertex(1), false);
        assert_eq!(ft.has_vertex(6), false);
        assert_eq!(ft.has_vertex(29), false);
        for i in 9..27 {
            assert_eq!(ft.has_vertex(i), false)
        }
        assert_eq!(ft.size(), 32);
        assert_eq!(ft.first_vertex().unwrap().index(), 0);
        let vertices_check = vec![0, 2, 3, 4, 5, 7, 8, 28, 30, 31];
        let mut vertices = vec![];
        for vertex in ft.vertices() {
            vertices.push(vertex.index());
        }
        assert_eq!(vertices, vertices_check);
    }

    #[test]
    fn should_be_vertex_disjoint_graphs() {
        let graph = test_data::get_falcon_graph();

        let ft1 = full_tree(&graph, 26, 1);
        let ft2 = full_tree(&graph, 5, 2);
        let vertex_disjoint = vertex_disjoint_graphs(&ft1, &ft2);
        assert_eq!(vertex_disjoint, true);

        let ft1 = full_tree(&graph, 26, 1);
        let ft2 = full_tree(&graph, 0, 2);
        let vertex_disjoint = vertex_disjoint_graphs(&ft1, &ft2);
        assert_eq!(vertex_disjoint, false);

        let ft1 = full_tree(&graph, 6, 1);
        let ft2 = full_tree(&graph, 5, 2);
        let vertex_disjoint = vertex_disjoint_graphs(&ft1, &ft2);
        assert_eq!(vertex_disjoint, false);

        let ft1 = full_tree(&graph, 6, 1);
        let ft2 = full_tree(&graph, 5, 1);
        let vertex_disjoint = vertex_disjoint_graphs(&ft1, &ft2);
        assert_eq!(vertex_disjoint, true);
    }

    #[test]
    fn should_contract_subgraph() {
        let graph = test_data::get_falcon_graph();

        let ft1 = full_tree(&graph, 26, 1);
        let ft2 = full_tree(&graph, 5, 2);
        let vertex_disjoint = vertex_disjoint_graphs(&ft1, &ft2);
        assert_eq!(vertex_disjoint, true);

        let mut contracted_graph = contract_sub_graph(&graph, &ft1);
        let source = contracted_graph.1;
        contracted_graph = contract_sub_graph(&contracted_graph.0, &ft2);
        let sink = contracted_graph.1;

        assert_eq!(source.is_some(), true);
        assert_eq!(source.unwrap(), 20);
        assert_eq!(sink.is_some(), true);
        assert_eq!(sink.unwrap(), 0);

        let mut contracted_graph = contracted_graph.0;
        let edges_check = vec![
            (0, 22),
            (0, 9),
            (0, 6),
            (0, 9),
            (0, 32),
            (0, 34),
            (0, 1),
            (0, 35),
        ];
        for edge_check in edges_check.iter() {
            assert_eq!(contracted_graph.has_edge(edge_check.0, edge_check.1), true);
            contracted_graph.remove_edge(edge_check.0, edge_check.1);
        }
        let neighbors_of_zero = contracted_graph.neighbors_of_vertex(0);
        assert_eq!(neighbors_of_zero.len(), 0);

        let edges_check = vec![(20, 22), (20, 23), (20, 24), (20, 25), (20, 12), (20, 10)];
        for edge_check in edges_check.iter() {
            assert_eq!(contracted_graph.has_edge(edge_check.0, edge_check.1), true);
            contracted_graph.remove_edge(edge_check.0, edge_check.1);
        }
        let neighbors_of_zero = contracted_graph.neighbors_of_vertex(20);
        assert_eq!(neighbors_of_zero.len(), 0);
    }
}
