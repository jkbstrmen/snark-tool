use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::undirected::edge::UndirectedEdge;
use crate::service::colour::colouriser::Colouriser;

pub struct RemovablePairsOfEdges<'a, G: Graph + Clone, C: Colouriser> {
    first_edge_iterator: Box<dyn Iterator<Item = &'a UndirectedEdge> + 'a>,
    second_edge_iterator: Box<dyn Iterator<Item = &'a UndirectedEdge> + 'a>,
    first_edge: Option<&'a UndirectedEdge>,
    graph: &'a G,
    local_graph: G,
    colouriser: &'a C,
}

impl<'a, G: Graph<E = UndirectedEdge> + Clone, C: Colouriser> Iterator
    for RemovablePairsOfEdges<'a, G, C>
{
    type Item = (&'a UndirectedEdge, &'a UndirectedEdge);

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_edge.is_none() {
            self.first_edge = self.first_edge_iterator.next();
        }

        while let Some(first_edge) = self.first_edge {
            while let Some(second_edge) = self.second_edge_iterator.next() {
                if first_edge.eq(second_edge) || first_edge > second_edge {
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

impl<'a, G: Graph<E = UndirectedEdge> + Clone, C: Colouriser> RemovablePairsOfEdges<'a, G, C> {
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

pub struct PairsOfNonAdjacentEdges<'a, G: Graph<E = UndirectedEdge>> {
    first_edge_iterator: Box<dyn Iterator<Item = &'a UndirectedEdge> + 'a>,
    second_edge_iterator: Box<dyn Iterator<Item = &'a UndirectedEdge> + 'a>,
    first_edge_current: Option<&'a UndirectedEdge>,
    graph: &'a G,
}

impl<'a, G: Graph<E = UndirectedEdge>> Iterator for PairsOfNonAdjacentEdges<'a, G> {
    type Item = (&'a UndirectedEdge, &'a UndirectedEdge);

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_edge_current.is_none() {
            self.first_edge_current = self.first_edge_iterator.next();
        }

        while let Some(first_edge) = self.first_edge_current {
            while let Some(second_edge) = self.second_edge_iterator.next() {
                if first_edge.eq(second_edge) {
                    continue;
                }

                if first_edge > second_edge {
                    continue;
                }

                if !first_edge.is_adjacent(second_edge) {
                    return Some((first_edge, second_edge));
                }
            }
            // renew second edge iterator to original value
            self.second_edge_iterator = self.graph.edges();
            self.first_edge_current = self.first_edge_iterator.next();
        }
        None
    }
}

impl<'a, G: Graph<E = UndirectedEdge>> PairsOfNonAdjacentEdges<'a, G> {
    pub fn new(graph: &'a G) -> Self {
        PairsOfNonAdjacentEdges {
            first_edge_iterator: graph.edges(),
            second_edge_iterator: graph.edges(),
            first_edge_current: None,
            graph,
        }
    }
}
