#[cfg(test)]
pub mod cyclic_edge_connectivity_tests {
    use crate::graph::graph::Graph;
    use crate::graph::vertex::Vertex;
    use crate::service::property::cyclic_connectivity::{
        cyclic_edge_connectivity, full_tree, vertex_disjoint_graphs,
    };
    use crate::test::test_data::test_data;

    #[test]
    fn should_have_cec_five() {
        let graph = test_data::get_petersen_graph();
        let cec = cyclic_edge_connectivity(&graph);
        assert_eq!(cec.is_some(), true);
        assert_eq!(cec.unwrap(), 5);
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



    }
}
