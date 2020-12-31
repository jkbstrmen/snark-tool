use crate::graph::graph::Graph;
use crate::graph::multi::graph::MultiGraph;
use crate::service::graph_traversal::bfs::BfsOfGraph;
use crate::service::property::max_flow::residual_graph::graph::ResidualGraph;
use std::cmp;

pub struct FordFulkerson {}

impl FordFulkerson {
    ///
    /// only use with undirected unweighted graphs
    ///
    pub fn max_flow(graph: &MultiGraph, source: usize, sink: usize) -> usize {
        let mut residual_graph = ResidualGraph::from_multi_graph(graph);
        let mut max_flow = 0;

        let residual_graph_ptr = &residual_graph as *const ResidualGraph;
        // find path in residual graph - while there is augmenting path do

        // when augmenting - BfsOfGraph is not using residual_graph anymore, just reading already discovered vertices
        // from its own attributes, hence use of raw pointer is safe here
        unsafe {
            while let Some(augmenting_path) =
                Self::find_augmenting_path(residual_graph_ptr, source, sink)
            {
                let path_flow = Self::augment(&mut residual_graph, augmenting_path, source, sink);
                max_flow += path_flow;
            }
        }
        max_flow
    }

    unsafe fn find_augmenting_path<'a>(
        residual_graph: *const ResidualGraph,
        source: usize,
        sink: usize,
    ) -> Option<BfsOfGraph<'a, ResidualGraph>> {
        let mut bfs = BfsOfGraph::new_from_raw_ptr(residual_graph, source);
        while let Some(next) = bfs.next() {
            if next.index() == sink {
                return Some(bfs);
            }
        }
        None
    }

    ///
    /// returns path flow
    ///
    fn augment(
        residual_graph: &mut ResidualGraph,
        path: BfsOfGraph<ResidualGraph>,
        source: usize,
        sink: usize,
    ) -> usize {
        // hence for now used only with non-weighted graphs - bottle neck is always 1
        let bottle_neck = 1;
        // hence used only inside of module - unsafe unwrap used
        let mut vertex = path.visited_vertex(sink).unwrap();
        loop {
            let before = vertex.discovered_from();
            residual_graph.decrease_edge_capacity(before, vertex.index());
            residual_graph.increase_edge_capacity(vertex.index(), before);
            if before == source {
                break;
            }
            vertex = path.visited_vertex(before).unwrap();
        }
        bottle_neck
    }
}
