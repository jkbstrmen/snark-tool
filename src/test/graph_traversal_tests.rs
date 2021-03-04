#[cfg(test)]
pub mod graph_traversal_tests {
    use crate::service::graph_traversal::bfs::BfsOfGraph;
    use crate::service::graph_traversal::dfs::DfsOfGraph;
    use crate::test::test_data::test_data;

    #[test]
    fn should_traverse_using_bfs() {
        let graph = test_data::get_petersen_graph();

        let mut bfs = BfsOfGraph::new(&graph, 0);

        let mut vertices = vec![];
        while let Some(next) = bfs.next() {
            vertices.push(next.index());
        }

        let right_order = vec![0, 4, 6, 8, 2, 5, 1, 7, 3, 9];
        assert_eq!(vertices, right_order);
    }

    #[test]
    fn should_reconstruct_path_using_bfs() {
        let graph = test_data::get_falcon_graph();
        let source = 5;
        let target = 6;
        let mut bfs = BfsOfGraph::new(&graph, 5);

        while let Some(next) = bfs.next() {
            if next.index() == target {
                break;
            }
        }
        let visited = bfs.visited_vertex(target);
        assert_eq!(visited.is_some(), true);

        let mut visited = visited.unwrap();
        let mut path = vec![];
        path.push(visited.index());
        loop {
            let before = visited.discovered_from();
            path.push(before);
            if before == source {
                break;
            }
            visited = bfs.visited_vertex(before).unwrap();
        }
        let path_check = vec![6, 7, 3, 5];
        assert_eq!(path, path_check);
    }

    #[test]
    fn should_traverse_using_bfs_discovery_order() {
        let graph = test_data::get_petersen_graph();

        let mut bfs = BfsOfGraph::new(&graph, 0);

        let mut vertices = vec![];
        while let Some(next) = bfs.next() {
            vertices.push(next.index());
        }

        let right_order = vec![0, 4, 6, 8, 2, 5, 1, 7, 3, 9];
        assert_eq!(vertices, right_order);
        assert_eq!(bfs.discovery_order(), &right_order);

        bfs.back();
        let right_order = vec![0, 4, 6, 8, 2, 5, 1, 7, 3];
        assert_eq!(bfs.discovery_order(), &right_order);

        bfs.back();
        bfs.back();
        let right_order = vec![0, 4, 6, 8, 2, 5, 1];
        assert_eq!(bfs.discovery_order(), &right_order);

        bfs.next();
        bfs.next();
        bfs.next();
        let right_order = vec![0, 4, 6, 8, 2, 5, 1, 7, 3, 9];
        assert_eq!(bfs.discovery_order(), &right_order);

        bfs.back();
        bfs.back();
        bfs.back();
        bfs.back();
        bfs.back();
        bfs.back();
        let right_order = vec![0, 4, 6, 8];
        assert_eq!(bfs.discovery_order(), &right_order);

        bfs.next();
        bfs.next();
        bfs.next();
        bfs.next();
        bfs.next();
        bfs.next();
        bfs.next();
        bfs.next();
        bfs.next();
        bfs.next();
        bfs.next();
        bfs.next();
        bfs.next();
        bfs.next();
        bfs.next();
        let right_order = vec![0, 4, 6, 8, 2, 5, 1, 7, 3, 9];
        assert_eq!(bfs.discovery_order(), &right_order);
    }

    #[test]
    fn should_traverse_using_dfs() {
        // TODO - finish assert

        let graph = test_data::get_petersen_graph();

        let mut dfs = DfsOfGraph::new(&graph, 0);

        let mut vertices = vec![];
        while let Some(next) = dfs.next() {
            println!("{}", next.index());

            vertices.push(next.index());
        }

        // let right_order = vec![0, 4, 6, 8, 2, 5, 1, 7, 3, 9];
        // assert_eq!(vertices, right_order);
    }
}
