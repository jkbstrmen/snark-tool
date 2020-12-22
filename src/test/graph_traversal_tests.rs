#[cfg(test)]
pub mod graph_traversal_tests {
    use crate::service::graph_traversal::bfs::BfsOfGraph;
    use crate::test::test_data::test_data;
    use serde::export::Option::Some;

    #[test]
    fn should_traverse_using_bfs() {
        let graph = test_data::get_petersen_graph();

        let mut bfs = BfsOfGraph::new(&graph, 0);

        let mut vertices = vec![];
        while let Some(next) = bfs.next() {
            vertices.push(next);
        }

        let right_order = vec![0, 4, 6, 8, 2, 5, 1, 7, 3, 9];
        assert_eq!(vertices, right_order);
    }
}
