use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::undirected::edge::UndirectedEdge;
use crate::service::colour::colouriser::Colourizer;

pub struct RemovablePairsOfEdges<'a, E: Edge, G: Graph + Clone, C: Colourizer> {
    first_edge_iterator: Box<dyn Iterator<Item = &'a E> + 'a>,
    second_edge_iterator: Box<dyn Iterator<Item = &'a E> + 'a>,
    first_edge: Option<&'a E>,
    graph: &'a G,
    local_graph: G,
    colouriser: &'a C,
}

impl<'a, E: Edge, G: Graph<E = E> + Clone, C: Colourizer> Iterator
    for RemovablePairsOfEdges<'a, E, G, C>
{
    type Item = (&'a E, &'a E);

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_edge.is_none() {
            self.first_edge = self.first_edge_iterator.next();
        }

        while let Some(first_edge) = self.first_edge {
            while let Some(second_edge) = self.second_edge_iterator.next() {
                if first_edge.eq(second_edge) {
                    continue;
                }

                self.local_graph
                    .remove_edge(first_edge.from(), first_edge.to());
                self.local_graph
                    .remove_edge(second_edge.from(), second_edge.to());

                let colourable = C::is_colorable(&self.local_graph);

                self.local_graph
                    .add_edge(first_edge.from(), first_edge.to());
                self.local_graph
                    .add_edge(second_edge.from(), second_edge.to());

                if !colourable {
                    return Some((first_edge, second_edge));
                }
            }
            // renew second edge iterator to original value
            self.second_edge_iterator = self.graph.edges();
            self.first_edge = self.first_edge_iterator.next();
        }
        None
    }
}

impl<'a, E: Edge, G: Graph<E = E> + Clone, C: Colourizer> RemovablePairsOfEdges<'a, E, G, C> {
    pub fn new(graph: &'a G, colouriser: &'a C) -> Self {
        RemovablePairsOfEdges {
            first_edge_iterator: graph.edges(),
            second_edge_iterator: graph.edges(),
            first_edge: None,
            graph,
            local_graph: graph.clone(),
            colouriser,
        }
    }
}

pub fn next_removable_pair_of_edges() {}
