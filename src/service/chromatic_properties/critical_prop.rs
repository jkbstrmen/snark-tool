use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;
use crate::service::colour::bfs::BFSColourizer;
use crate::service::colour::colouriser::Colourizer;

pub struct CriticalProperties<G, C>
where
    G: Graph + Clone,
    C: Colourizer,
{
    graph: G,
    colourizer: C,

    is_critical: bool,
    is_cocritical: bool,
    is_vertex_subcritical: bool,
    is_edge_subcritical: bool,

    colourings: Vec<Option<bool>>,
    computed: bool,
}

impl<G, C> CriticalProperties<G, C>
where
    G: Graph + Clone,
    C: Colourizer,
{
    pub fn of_graph_with_colourizer(graph: &G, colourizer: C) -> Self {
        CriticalProperties {
            graph: graph.clone(),
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
    fn compute_vertex_properties(&mut self){

        // TODO

        self.is_critical = true;
        self.is_cocritical = true;

        let graph = &self.graph;

        // let vertices = self.graph.vertices();

        for first_vertex in graph.vertices() {
            self.is_vertex_subcritical = false;

            // remove edges of first vertex
            // graph.remove_edges_of_vertex(first_vertex.index());

            for second_vertex in graph.vertices() {
                if first_vertex == second_vertex {
                    continue;
                }

                // skip unnecessary tests
                if self.graph.has_edge(first_vertex.index(), second_vertex.index()) && !self.is_critical && self.is_vertex_subcritical {
                    continue;
                }
                if !self.graph.has_edge(first_vertex.index(), second_vertex.index()) && !self.is_cocritical && self.is_vertex_subcritical {
                    continue;
                }

                let colourable_opt = self.colourings[first_vertex.index() * self.graph.size() + second_vertex.index()];
                let colourable;
                if colourable_opt.is_some() {
                    colourable = colourable_opt.unwrap();
                } else {

                    // remove edges of second_vertex

                    // self.colourizer.

                    // set colouring
                }

                // produce consequences
                // if (!colourable) {
                //     if (graph_.isEdge(firstVertex, secondVertex)) {
                //         isCritical_ = false;
                //     } else {
                //         isCocritical_ = false;
                //     }
                // } else {
                //     isVertexSubcritical_ = true;
                // }


            }


            // if (!isVertexSubcritical_) return;
        }

    }

    fn compute_edge_properties(&mut self){

        // TODO

    }
}

impl<G> CriticalProperties<G, BFSColourizer>
where
    G: Graph + Clone,
{
    pub fn of_graph(graph: &G) -> Self {
        CriticalProperties::<G, BFSColourizer>::of_graph_with_colourizer(
            graph,
            BFSColourizer::new(),
        )
    }
}
