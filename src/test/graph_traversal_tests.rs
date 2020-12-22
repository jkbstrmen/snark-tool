#[cfg(test)]
pub mod graph_traversal_tests {
    use crate::service::graph_traversal::bfs::BfsOfGraph;
    use crate::test::test_data::test_data;

    #[test]
    fn should_traverse_using_bfs() {
        let graph = test_data::get_petersen_graph();

        let mut bfs = BfsOfGraph::new(&graph, 0);
        bfs.bfs_next();
    }
}
