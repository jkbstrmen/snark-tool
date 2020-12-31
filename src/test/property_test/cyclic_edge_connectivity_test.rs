#[cfg(test)]
pub mod cyclic_edge_connectivity_tests {
    use crate::graph::graph::{Graph, GraphConstructor};
    use crate::graph::undirected::simple_graph::graph::SimpleGraph;
    use crate::graph::vertex::Vertex;
    use crate::service::io::reader::Reader;
    use crate::service::io::reader_g6::G6Reader;
    use crate::service::property::cyclic_connectivity::{
        contract_sub_graph, cyclic_edge_connectivity, full_tree, vertex_disjoint_graphs,
    };
    use crate::test::test_data::test_data;
    use std::fs;

    #[test]
    fn should_have_cec_five() {
        let graph = test_data::get_petersen_graph();
        let cec = cyclic_edge_connectivity(&graph);
        assert_eq!(cec.is_some(), true);
        assert_eq!(cec.unwrap(), 5);

        let graph_string =
            "]D?O@S??G??@??B?g??OP_G@????O?C?C_A?@?GG@?A??_`_??_?`??_AO????G@OC?????G?W";
        let graph: SimpleGraph = G6Reader::read_graph(graph_string).unwrap();
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
