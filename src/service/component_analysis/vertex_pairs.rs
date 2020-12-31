use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;

///
/// de-facto list of edges for simple graph with undirected edges
///
pub struct PairsOfAdjacentVertices<'a, V: Vertex, G: Graph> {
    first_vertex_iterator: Box<dyn Iterator<Item = &'a V> + 'a>,
    second_vertex_iterator: Box<dyn Iterator<Item = &'a V> + 'a>,
    first_vertex_current: Option<&'a V>,
    graph: &'a G,
}

impl<'a, V: Vertex, G: Graph<V = V>> Iterator for PairsOfAdjacentVertices<'a, V, G> {
    type Item = (&'a V, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_vertex_current.is_none() {
            self.first_vertex_current = self.first_vertex_iterator.next();
        }
        while let Some(first_vertex) = self.first_vertex_current {
            while let Some(second_vertex) = self.second_vertex_iterator.next() {
                if first_vertex.index().eq(&second_vertex.index()) {
                    continue;
                }
                if first_vertex.index() > second_vertex.index() {
                    continue;
                }
                if self
                    .graph
                    .has_edge(first_vertex.index(), second_vertex.index())
                {
                    return Some((first_vertex, second_vertex));
                }
            }
            // renew second edge iterator to original value
            self.second_vertex_iterator = self.graph.vertices();
            self.first_vertex_current = self.first_vertex_iterator.next();
        }
        None
    }
}

impl<'a, V: Vertex, G: Graph<V = V>> PairsOfAdjacentVertices<'a, V, G> {
    pub fn new(graph: &'a G) -> Self {
        PairsOfAdjacentVertices {
            first_vertex_iterator: graph.vertices(),
            second_vertex_iterator: graph.vertices(),
            first_vertex_current: None,
            graph,
        }
    }
}
