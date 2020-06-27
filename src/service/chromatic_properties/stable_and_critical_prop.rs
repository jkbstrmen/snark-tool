use crate::graph::edge::{Edge, EdgeConstructor};
use crate::graph::graph::Graph;
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::undirected::simple_graph::SimpleGraph;
use crate::graph::undirected_sparse::graph::SimpleSparseGraph;
use crate::graph::vertex::Vertex;
use crate::service::colour::colouriser::Colourizer;

// TODO - use own graph struct?

pub struct StableAndCriticalProperties<C>
where
    C: Colourizer,
{
    untouched_graph: SimpleSparseGraph,
    graph: SimpleSparseGraph,
    _colourizer: C,

    is_critical: bool,
    is_cocritical: bool,
    is_vertex_subcritical: bool,
    is_edge_subcritical: bool,
    is_stable: bool,
    is_costable: bool,

    colourings: Vec<Option<bool>>,
    computed: bool,
    results_obtained: bool,
}

impl<C> StableAndCriticalProperties<C>
where
    C: Colourizer,
{
    pub fn of_graph_with_colourizer<G: Graph + Clone>(graph: &G, colourizer: C) -> Self {
        let local_graph = SimpleSparseGraph::from_graph(graph);
        StableAndCriticalProperties {
            untouched_graph: local_graph.clone(),
            graph: local_graph,
            _colourizer: colourizer,
            is_critical: false,
            is_cocritical: false,
            is_vertex_subcritical: false,
            is_edge_subcritical: false,
            is_stable: false,
            is_costable: false,
            colourings: vec![None; graph.size() * graph.size()],
            computed: false,
            results_obtained: false,
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

    pub fn is_stable(&mut self) -> bool {
        if self.computed {
            return self.is_stable;
        }
        self.compute_properties();
        return self.is_stable;
    }

    pub fn is_costable(&mut self) -> bool {
        if self.computed {
            return self.is_costable;
        }
        self.compute_properties();
        return self.is_costable;
    }

    fn compute_properties(&mut self) {
        self.compute_vertex_properties();

        self.is_edge_subcritical = true;
        if !self.is_critical {
            self.compute_edge_subcriticality();
        }
        self.computed = true;
    }

    ///
    /// Compute criticality, cocriticality and vertex subcriticality of graph
    ///
    fn compute_vertex_properties(&mut self) {
        self.is_critical = true;
        self.is_cocritical = true;
        self.is_vertex_subcritical = true;

        self.is_stable = true;
        self.is_costable = true;

        let graph = &mut self.graph;
        for first_vertex in 0..graph.size {
            graph.remove_edges_of_vertex(first_vertex);

            for second_vertex in 0..graph.size() {
                if first_vertex == second_vertex {
                    continue;
                }

                let colourable_opt = self.colourings[first_vertex * graph.size() + second_vertex];
                let colourable;
                if colourable_opt.is_none() {
                    graph.remove_edges_of_vertex(second_vertex);

                    colourable = C::is_colorable(graph);

                    self.colourings[first_vertex * graph.size() + second_vertex] = Some(colourable);
                    Self::restore_edges_of_vertex_except_for(
                        &self.untouched_graph,
                        graph,
                        second_vertex,
                        first_vertex,
                    );
                }
            }

            // HERE todo
            // self.check_properties(first_vertex);

            // check properties
            {
                let mut vertex_subcritical_flag = false;

                for second_vertex in self.untouched_graph.vertices.iter() {
                    if first_vertex == second_vertex.index() {
                        continue;
                    }

                    let colourings = &self.colourings;

                    if self
                        .untouched_graph
                        .has_edge(first_vertex, second_vertex.index())
                    {
                        // if self.get_colouring(first_vertex, second_vertex.index()) {
                        if Self::get_colouring(
                            colourings,
                            graph.size,
                            first_vertex,
                            second_vertex.index(),
                        ) {
                            self.is_stable = false;
                            vertex_subcritical_flag = true;
                        } else {
                            self.is_critical = false;
                        }
                    } else {
                        if Self::get_colouring(
                            colourings,
                            graph.size,
                            first_vertex,
                            second_vertex.index(),
                        ) {
                            self.is_costable = false;
                            vertex_subcritical_flag = true;
                        } else {
                            self.is_cocritical = false;
                        }
                    }
                }

                if !vertex_subcritical_flag {
                    self.is_vertex_subcritical = false;
                }

                if !self.is_vertex_subcritical
                    && !self.is_critical
                    && !self.is_cocritical
                    && !self.is_stable
                    && !self.is_costable
                {
                    self.results_obtained = true;
                }
            }

            Self::restore_edges_of_vertex(&self.untouched_graph, graph, first_vertex);

            if self.results_obtained {
                return;
            };
        }
    }

    // fn check_properties(&mut self, vertex: usize) {
    //     let mut vertex_subcritical_flag = false;
    //
    //     for second_vertex in self.untouched_graph.vertices.iter() {
    //         if vertex == second_vertex.index() {
    //             continue;
    //         }
    //
    //         if self.untouched_graph.has_edge(vertex, second_vertex.index()) {
    //             if self.get_colouring_v2(vertex, second_vertex.index()) {
    //                 self.is_stable = false;
    //                 vertex_subcritical_flag = true;
    //             } else {
    //                 self.is_critical = false;
    //             }
    //         } else {
    //             if self.get_colouring_v2(vertex, second_vertex.index()) {
    //                 self.is_costable = false;
    //                 vertex_subcritical_flag = true;
    //             } else {
    //                 self.is_critical = false;
    //             }
    //         }
    //     }
    //
    //     if !vertex_subcritical_flag {
    //         self.is_vertex_subcritical = false;
    //     }
    //
    //     if !self.is_vertex_subcritical
    //         && !self.is_critical
    //         && !self.is_cocritical
    //         && !self.is_stable
    //         && !self.is_costable
    //     {
    //         self.results_obtained = true;
    //     }
    // }

    // fn get_colouring_v2(&self, from: usize, to: usize) -> bool {
    //     return self.colourings[from * self.graph.size + to].unwrap();
    // }

    fn get_colouring(
        colourings: &Vec<Option<bool>>,
        graph_size: usize,
        from: usize,
        to: usize,
    ) -> bool {
        return colourings[from * graph_size + to].unwrap();
    }

    // todo - move above - same here, same in critic. props
    fn compute_edge_subcriticality(&mut self) {
        let local_graph = SimpleGraph::from_graph(&self.graph);
        let mut edge_subcritical = true;

        for first_edge in local_graph.edges.iter() {
            self.graph.remove_edge(first_edge.from(), first_edge.to());

            for second_edge in local_graph.edges.iter() {
                if first_edge.eq(second_edge) {
                    continue;
                }
                self.graph.remove_edge(second_edge.from(), second_edge.to());

                let colourable = C::is_colorable(&self.graph);

                self.graph.add_edge(second_edge.from(), second_edge.to());
                if colourable {
                    edge_subcritical = true;
                    break;
                }
                edge_subcritical = false;
            }
            self.graph.add_edge(first_edge.from(), first_edge.to());
            if !edge_subcritical {
                self.is_edge_subcritical = false;
                return;
            }
        }
        self.is_edge_subcritical = true;
    }

    fn restore_edges_of_vertex(
        original_graph: &SimpleSparseGraph,
        changed_graph: &mut SimpleSparseGraph,
        vertex: usize,
    ) {
        for neighboring_edge in original_graph.vertices[vertex].edges.iter() {
            changed_graph.add_edge(neighboring_edge.from(), neighboring_edge.to());
        }
    }

    fn restore_edges_of_vertex_except_for(
        original_graph: &SimpleSparseGraph,
        changed_graph: &mut SimpleSparseGraph,
        vertex: usize,
        except_for: usize,
    ) {
        let except_for_edge = UndirectedEdge::new(vertex, except_for);
        for neighboring_edge in original_graph.vertices[vertex].edges.iter() {
            if neighboring_edge.from() == except_for_edge.from()
                && neighboring_edge.to() == except_for_edge.to()
            {
                continue;
            }
            changed_graph.add_edge(neighboring_edge.from(), neighboring_edge.to());
        }
    }
}