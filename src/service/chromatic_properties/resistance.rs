use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;
use crate::service::colour::bfs::BFSColourizer;
use crate::service::colour::colouriser::Colourizer;
use std::marker;

pub struct Resistance<G, V, E, C>
where
    G: Graph<V, E>,
    V: Vertex,
    E: Edge,
    C: Colourizer,
{
    _g: marker::PhantomData<G>,
    _v: marker::PhantomData<V>,
    _e: marker::PhantomData<E>,
    colourizer: C,
}

impl<G, V, E, C> Resistance<G, V, E, C>
where
    G: Graph<V, E>,
    V: Vertex,
    E: Edge,
    C: Colourizer,
{
    pub fn new() -> Self {
        Resistance {
            _g: marker::PhantomData,
            _v: marker::PhantomData,
            _e: marker::PhantomData,
            colourizer: C::new(),
        }
    }

    pub fn new_with_colourizer(colourizer: C) -> Self {
        Resistance {
            _g: marker::PhantomData,
            _v: marker::PhantomData,
            _e: marker::PhantomData,
            colourizer,
        }
    }

    pub fn edge_resistance(&self, graph: &G) -> Option<usize> {
        for i in 1..graph.size() {
            let res = self.edge_resistance_recursive(graph, i);
            if res.is_some() {
                return res;
            }
        }
        None
    }

    fn edge_resistance_recursive(&self, graph: &G, max_nesting: usize) -> Option<usize> {
        if max_nesting == 0 {
            let colourable = C::is_colorable(graph);
            if colourable {
                return Some(0);
            }
            return None;
        }

        let mut local_graph = graph.clone();
        for edge in graph.edges() {
            local_graph.remove_edge(edge.from(), edge.to());
            let res = self.edge_resistance_recursive(&local_graph, max_nesting - 1);

            if res.is_some() {
                return Some(res.unwrap() + 1);
            }
            local_graph.add_edge(edge.from(), edge.to());
        }
        None
    }

    pub fn vertex_resistance(&self, graph: &G) -> usize {
        0
    }
}
