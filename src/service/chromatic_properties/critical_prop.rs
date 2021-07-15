use crate::graph::edge::{Edge, EdgeConstructor};
use crate::graph::graph::Graph;
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::undirected::simple_graph::graph::SimpleGraph;
use crate::service::chromatic_properties::CriticalProperties;
use crate::service::colour::colouriser::Colouriser;
use crate::service::colour::recursive::dfs_improved::DFSColourizer;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CriticalPropertiesStruct {
    pub untouched_graph: SimpleGraph,
    pub graph: SimpleGraph,

    pub is_critical: bool,
    pub is_cocritical: bool,
    pub is_vertex_subcritical: bool,
    pub is_edge_subcritical: bool,

    pub colourings: Vec<Option<bool>>,
    pub vertex_properties_computed: bool,
    pub edge_property_computed: bool,
}

impl CriticalPropertiesStruct {
    pub fn of_graph<G: Graph + Clone>(graph: &G) -> Self {
        let local_graph = SimpleGraph::from_graph(graph);
        Self {
            untouched_graph: local_graph.clone(),
            graph: local_graph,
            is_critical: false,
            is_cocritical: false,
            is_vertex_subcritical: false,
            is_edge_subcritical: false,
            colourings: vec![None; graph.size() * graph.size()],
            vertex_properties_computed: false,
            edge_property_computed: false,
        }
    }
}

pub struct CriticalPropertiesSolver<C>
where
    C: Colouriser,
{
    _colourizer: C,
    properties: CriticalPropertiesStruct,
}

impl<C> CriticalProperties for CriticalPropertiesSolver<C>
where
    C: Colouriser,
{
    fn is_critical(&mut self) -> bool {
        if self.properties.vertex_properties_computed {
            return self.properties.is_critical;
        }
        self.compute_properties();
        return self.properties.is_critical;
    }

    fn is_cocritical(&mut self) -> bool {
        if self.properties.vertex_properties_computed {
            return self.properties.is_cocritical;
        }
        self.compute_properties();
        return self.properties.is_cocritical;
    }

    fn is_vertex_subcritical(&mut self) -> bool {
        if self.properties.vertex_properties_computed {
            return self.properties.is_vertex_subcritical;
        }
        self.compute_properties();
        return self.properties.is_vertex_subcritical;
    }

    fn is_edge_subcritical(&mut self) -> bool {
        if self.properties.edge_property_computed {
            return self.properties.is_edge_subcritical;
        }
        self.properties.is_edge_subcritical =
            Self::compute_edge_subcriticality(&mut self.properties.graph);
        self.properties.edge_property_computed = true;

        return self.properties.is_edge_subcritical;
    }

    fn is_acritical(&mut self) -> bool {
        !self.is_vertex_subcritical()
    }
}

impl<C> CriticalPropertiesSolver<C>
where
    C: Colouriser,
{
    pub fn of_graph_with_colourizer<G: Graph + Clone>(graph: &G, colourizer: C) -> Self {
        CriticalPropertiesSolver {
            _colourizer: colourizer,
            properties: CriticalPropertiesStruct::of_graph(graph),
        }
    }

    fn compute_properties(&mut self) {
        self.compute_vertex_properties();

        if self.properties.is_critical {
            self.properties.is_edge_subcritical = true;
            self.properties.edge_property_computed = true;
        }
        self.properties.vertex_properties_computed = true;
    }

    ///
    /// Compute criticality, cocriticality and vertex subcriticality of graph
    ///
    fn compute_vertex_properties(&mut self) {
        self.properties.is_critical = true;
        self.properties.is_cocritical = true;

        let graph = &mut self.properties.graph;

        for first_vertex in 0..graph.size() {
            self.properties.is_vertex_subcritical = false;

            graph.remove_edges_of_vertex(first_vertex);

            for second_vertex in 0..graph.size() {
                if first_vertex == second_vertex {
                    continue;
                }

                // skip unnecessary tests
                if self
                    .properties
                    .untouched_graph
                    .has_edge(first_vertex, second_vertex)
                    && !self.properties.is_critical
                    && self.properties.is_vertex_subcritical
                {
                    continue;
                }
                if !self
                    .properties
                    .untouched_graph
                    .has_edge(first_vertex, second_vertex)
                    && !self.properties.is_cocritical
                    && self.properties.is_vertex_subcritical
                {
                    continue;
                }

                let colourable_opt =
                    self.properties.colourings[first_vertex * graph.size() + second_vertex];
                let colourable;
                if colourable_opt.is_some() {
                    colourable = colourable_opt.unwrap();
                } else {
                    graph.remove_edges_of_vertex(second_vertex);

                    colourable = C::is_colorable(graph);

                    self.properties.colourings[first_vertex * graph.size() + second_vertex] =
                        Some(colourable);
                    self.properties.colourings[second_vertex * graph.size() + first_vertex] =
                        Some(colourable);

                    restore_edges_of_vertex_except_for(
                        &self.properties.untouched_graph,
                        graph,
                        second_vertex,
                        first_vertex,
                    );
                }

                // check properties
                if !colourable {
                    if self
                        .properties
                        .untouched_graph
                        .has_edge(first_vertex, second_vertex)
                    {
                        self.properties.is_critical = false;
                    } else {
                        self.properties.is_cocritical = false;
                    }
                } else {
                    self.properties.is_vertex_subcritical = true;
                }
            }

            restore_edges_of_vertex(&self.properties.untouched_graph, graph, first_vertex);

            if !self.properties.is_vertex_subcritical {
                return;
            }
        }
    }

    pub fn compute_edge_subcriticality(graph: &mut SimpleGraph) -> bool {
        let local_graph = SimpleGraph::from_graph(graph);
        let mut edge_subcritical = true;

        let mut computed_pairs = HashMap::new();
        for first_edge in local_graph.edges() {
            graph.remove_edge(first_edge.from(), first_edge.to());

            for second_edge in local_graph.edges() {
                if first_edge.eq(second_edge) {
                    continue;
                }
                let colourable: bool;
                if computed_pairs.contains_key(&(first_edge, second_edge)) {
                    colourable = *computed_pairs.get(&(first_edge, second_edge)).unwrap();
                } else {
                    graph.remove_edge(second_edge.from(), second_edge.to());
                    colourable = C::is_colorable(graph);
                    graph.add_edge(second_edge.from(), second_edge.to());

                    computed_pairs.insert((first_edge, second_edge), colourable);
                    computed_pairs.insert((second_edge, first_edge), colourable);
                }
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

impl CriticalPropertiesSolver<DFSColourizer> {
    #[allow(dead_code)]
    pub fn of_graph<G: Graph + Clone>(graph: &G) -> Self {
        CriticalPropertiesSolver::<DFSColourizer>::of_graph_with_colourizer(
            graph,
            DFSColourizer::new(),
        )
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
