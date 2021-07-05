use crate::graph::edge::{Edge, EdgeConstructor};
use crate::graph::graph::Graph;
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::undirected::simple_graph::graph::SimpleGraph;
use crate::service::colour::colouriser::Colouriser;
use crate::service::colour::recursive::dfs_improved::DFSColourizer;

pub struct CriticalProperties<C>
where
    C: Colouriser,
{
    untouched_graph: SimpleGraph,
    graph: SimpleGraph,
    _colourizer: C,

    is_critical: bool,
    is_cocritical: bool,
    is_vertex_subcritical: bool,
    is_edge_subcritical: bool,

    colourings: Vec<Option<bool>>,
    vertex_properties_computed: bool,
    edge_property_computed: bool,
}

impl<C> CriticalProperties<C>
where
    C: Colouriser,
{
    pub fn of_graph_with_colourizer<G: Graph + Clone>(graph: &G, colourizer: C) -> Self {
        let local_graph = SimpleGraph::from_graph(graph);
        CriticalProperties {
            untouched_graph: local_graph.clone(),
            graph: local_graph,
            _colourizer: colourizer,
            is_critical: false,
            is_cocritical: false,
            is_vertex_subcritical: false,
            is_edge_subcritical: false,
            colourings: vec![None; graph.size() * graph.size()],
            vertex_properties_computed: false,
            edge_property_computed: false,
        }
    }

    pub fn is_critical(&mut self) -> bool {
        if self.vertex_properties_computed {
            return self.is_critical;
        }
        self.compute_properties();
        return self.is_critical;
    }

    pub fn is_cocritical(&mut self) -> bool {
        if self.vertex_properties_computed {
            return self.is_cocritical;
        }
        self.compute_properties();
        return self.is_cocritical;
    }

    pub fn is_vertex_subcritical(&mut self) -> bool {
        if self.vertex_properties_computed {
            return self.is_vertex_subcritical;
        }
        self.compute_properties();
        return self.is_vertex_subcritical;
    }

    pub fn is_edge_subcritical(&mut self) -> bool {
        if self.edge_property_computed {
            return self.is_edge_subcritical;
        }
        self.is_edge_subcritical = Self::compute_edge_subcriticality(&mut self.graph);
        self.edge_property_computed = true;

        return self.is_edge_subcritical;
    }

    pub fn is_acritical(&mut self) -> bool {
        !self.is_vertex_subcritical()
    }

    fn compute_properties(&mut self) {
        self.compute_vertex_properties();

        if self.is_critical {
            self.is_edge_subcritical = true;
            self.edge_property_computed = true;
        }
        self.vertex_properties_computed = true;
    }

    ///
    /// Compute criticality, cocriticality and vertex subcriticality of graph
    ///
    fn compute_vertex_properties(&mut self) {
        self.is_critical = true;
        self.is_cocritical = true;

        let graph = &mut self.graph;

        for first_vertex in 0..graph.size() {
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
                let colourable;
                if colourable_opt.is_some() {
                    colourable = colourable_opt.unwrap();
                } else {
                    graph.remove_edges_of_vertex(second_vertex);

                    colourable = C::is_colorable(graph);

                    self.colourings[first_vertex * graph.size() + second_vertex] = Some(colourable);
                    self.colourings[second_vertex * graph.size() + first_vertex] = Some(colourable);

                    restore_edges_of_vertex_except_for(
                        &self.untouched_graph,
                        graph,
                        second_vertex,
                        first_vertex,
                    );
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

            restore_edges_of_vertex(&self.untouched_graph, graph, first_vertex);

            if !self.is_vertex_subcritical {
                return;
            }
        }
    }

    pub fn compute_edge_subcriticality(graph: &mut SimpleGraph) -> bool {
        let local_graph = SimpleGraph::from_graph(graph);
        let mut edge_subcritical = true;

        // TODO - optimization
        // TODO - avoid computing for repeating pairs - pair (0, 1), (3, 4) is the same as (3, 4), (0, 1)

        for first_edge in local_graph.edges() {
            graph.remove_edge(first_edge.from(), first_edge.to());

            for second_edge in local_graph.edges() {
                if first_edge.eq(second_edge) {
                    continue;
                }
                graph.remove_edge(second_edge.from(), second_edge.to());
                let colourable = C::is_colorable(graph);
                graph.add_edge(second_edge.from(), second_edge.to());
                if colourable {
                    edge_subcritical = true;
                    break;
                }
                edge_subcritical = false;
            }
            graph.add_edge(first_edge.from(), first_edge.to());
            if !edge_subcritical {
                return false;
            }
        }
        true
    }
}

impl CriticalProperties<DFSColourizer> {
    #[allow(dead_code)]
    pub fn of_graph<G: Graph + Clone>(graph: &G) -> Self {
        CriticalProperties::<DFSColourizer>::of_graph_with_colourizer(graph, DFSColourizer::new())
    }
}

pub fn restore_edges_of_vertex(
    original_graph: &SimpleGraph,
    changed_graph: &mut SimpleGraph,
    vertex: usize,
) {
    for neighboring_edge in original_graph.vertices[vertex].edges.iter() {
        changed_graph.add_edge(neighboring_edge.from(), neighboring_edge.to());
    }
}

pub fn restore_edges_of_vertex_except_for(
    original_graph: &SimpleGraph,
    changed_graph: &mut SimpleGraph,
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
