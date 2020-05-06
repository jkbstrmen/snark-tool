use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::undirected::simple_graph::SimpleGraph;
use crate::graph::vertex::Vertex;
use crate::service::colour::bfs::BFSColourizer;
use crate::service::colour::colouriser::Colourizer;
use crate::graph::undirected_sparse::graph::SimpleSparseGraph;

// TODO - use own graph struct?

pub struct CriticalProperties<C>
where
    C: Colourizer,
{
    untouched_graph: SimpleSparseGraph,
    graph: SimpleSparseGraph,
    colourizer: C,

    is_critical: bool,
    is_cocritical: bool,
    is_vertex_subcritical: bool,
    is_edge_subcritical: bool,

    colourings: Vec<Option<bool>>,
    computed: bool,
}

impl<C> CriticalProperties<C>
where
    C: Colourizer,
{
    pub fn of_graph_with_colourizer<G: Graph + Clone >(graph: &G, colourizer: C) -> Self {
        let local_graph = SimpleSparseGraph::from_graph(graph);
        CriticalProperties {
            untouched_graph: local_graph.clone(),
            graph: local_graph,
            colourizer,
            is_critical: false,
            is_cocritical: false,
            is_vertex_subcritical: false,
            is_edge_subcritical: false,
            colourings: vec![None; graph.size() * graph.size()],
            computed: false,
        }
    }

    pub fn is_critical(&mut self) -> bool {
        if self.computed {
            return self.is_critical;
        }
        self.compute_properties();
        return self.is_critical;
    }

    pub fn is_cocritical(&mut self) -> bool {
        if self.computed {
            return self.is_cocritical;
        }
        self.compute_properties();
        return self.is_cocritical;
    }

    pub fn is_vertex_subcritical(&mut self) -> bool {
        if self.computed {
            return self.is_vertex_subcritical;
        }
        self.compute_properties();
        return self.is_vertex_subcritical;
    }

    pub fn is_edge_subcritical(&mut self) -> bool {
        if self.computed {
            return self.is_edge_subcritical;
        }
        self.compute_properties();
        return self.is_edge_subcritical;
    }

    fn compute_properties(&mut self) {
        self.compute_vertex_properties();

        self.is_edge_subcritical = true;
        if !self.is_critical {
            self.compute_edge_properties();
        }
        self.computed = true;
    }

    ///
    /// Compute criticality, cocriticality and vertex subcriticality of graph
    ///
    fn compute_vertex_properties(&mut self) {
        self.is_critical = true;
        self.is_cocritical = true;

        let graph = &mut self.graph;

        for first_vertex in 0..graph.size {
            self.is_vertex_subcritical = false;

            graph.remove_edges_of_vertex(first_vertex);

            for second_vertex in 0..graph.size() {
                if first_vertex == second_vertex {
                    continue;
                }

                // skip unnecessary tests
                if graph.has_edge(first_vertex, second_vertex)
                    && !self.is_critical
                    && self.is_vertex_subcritical
                {
                    continue;
                }
                if !graph.has_edge(first_vertex, second_vertex)
                    && !self.is_cocritical
                    && self.is_vertex_subcritical
                {
                    continue;
                }

                let colourable_opt = self.colourings[first_vertex * graph.size() + second_vertex];
                let mut colourable= false;
                if colourable_opt.is_some() {
                    colourable = colourable_opt.unwrap();
                } else {
                    // remove edges of second_vertex
                    graph.remove_edges_of_vertex(second_vertex);

                    // TODO
                    // colourable = C::is_colorable(graph);

                    self.colourings[first_vertex * graph.size() + second_vertex] = Some(colourable);

                    // revert edges removal
                    Self::restore_edges_of_vertex_except_for(&self.untouched_graph, graph, second_vertex, first_vertex);
                }

                // check properties
                if !colourable {
                    if self.untouched_graph.has_edge(first_vertex, second_vertex) {
                        self.is_critical = false;
                    } else {
                        self.is_cocritical = false;
                    }
                } else {
                    self.is_vertex_subcritical = true;
                }
            }

            Self::restore_edges_of_vertex(&self.untouched_graph, graph, first_vertex);

            if !self.is_vertex_subcritical {
                return;
            }
        }
    }

    fn compute_edge_properties(&mut self) {

        // TODO
    }

    fn restore_edges_of_vertex(original_graph: &SimpleSparseGraph, changed_graph: &mut SimpleSparseGraph, vertex: usize){
        for neighbor in original_graph.vertices[vertex].neighbors.iter() {
            changed_graph.add_edge(vertex, neighbor.index());
        }
    }

    fn restore_edges_of_vertex_except_for(original_graph: &SimpleSparseGraph, changed_graph: &mut SimpleSparseGraph, vertex: usize, except_for: usize){
        for neighbor in original_graph.vertices[vertex].neighbors.iter() {
            if neighbor.index() == except_for {
                continue;
            }
            changed_graph.add_edge(vertex, neighbor.index());
        }
    }

}

impl CriticalProperties<BFSColourizer>
{
    pub fn of_graph<G: Graph + Clone>(graph: &G) -> Self {
        CriticalProperties::<BFSColourizer>::of_graph_with_colourizer(graph, BFSColourizer::new())
    }
}
