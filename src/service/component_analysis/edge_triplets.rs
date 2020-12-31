use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::service::colour::colouriser::Colouriser;
use crate::graph::undirected::edge::UndirectedEdge;

pub struct RemovableTripletsOfEdges<'a, G: Graph + Clone, C: Colouriser> {
    first_edge_iterator: Box<dyn Iterator<Item = &'a UndirectedEdge> + 'a>,
    second_edge_iterator: Box<dyn Iterator<Item = &'a UndirectedEdge> + 'a>,
    third_edge_iterator: Box<dyn Iterator<Item = &'a UndirectedEdge> + 'a>,
    first_edge_current: Option<&'a UndirectedEdge>,
    second_edge_current: Option<&'a UndirectedEdge>,
    graph: &'a G,
    local_graph: G,
    colouriser: &'a C,
}

impl<'a, G: Graph<E = UndirectedEdge> + Clone, C: Colouriser> Iterator
    for RemovableTripletsOfEdges<'a, G, C>
{
    type Item = (&'a UndirectedEdge, &'a UndirectedEdge, &'a UndirectedEdge);

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_edge_current.is_none() {
            self.first_edge_current = self.first_edge_iterator.next();
        }
        if self.second_edge_current.is_none() {
            self.second_edge_current = self.second_edge_iterator.next();
        }

        while let Some(first_edge) = self.first_edge_current {
            while let Some(second_edge) = self.second_edge_current {
                if first_edge.eq(second_edge) || second_edge < first_edge {
                    self.second_edge_current = self.second_edge_iterator.next();
                    continue;
                }
                while let Some(third_edge) = self.third_edge_iterator.next() {
                    if second_edge.eq(third_edge)
                        || first_edge.eq(third_edge)
                        || third_edge < second_edge
                    {
                        continue;
                    }
                    self.local_graph
                        .remove_edge(first_edge.from(), first_edge.to());
                    self.local_graph
                        .remove_edge(second_edge.from(), second_edge.to());
                    self.local_graph
                        .remove_edge(third_edge.from(), third_edge.to());

                    let colourable = C::is_colorable(&self.local_graph);

                    self.local_graph
                        .add_edge(first_edge.from(), first_edge.to());
                    self.local_graph
                        .add_edge(second_edge.from(), second_edge.to());
                    self.local_graph
                        .add_edge(third_edge.from(), third_edge.to());

                    if !colourable {
                        return Some((first_edge, second_edge, third_edge));
                    }
                }
                // renew third edge iterator to original value
                self.third_edge_iterator = self.graph.edges();
                // shift second edge to next
                self.second_edge_current = self.second_edge_iterator.next();
            }
            // renew second edge iterator to original value and take first next
            self.second_edge_iterator = self.graph.edges();
            self.second_edge_current = self.second_edge_iterator.next();
            // shift first edge to next
            self.first_edge_current = self.first_edge_iterator.next();
        }
        None
    }
}

impl<'a, G: Graph<E = UndirectedEdge> + Clone, C: Colouriser> RemovableTripletsOfEdges<'a, G, C> {
    pub fn new(graph: &'a G, colouriser: &'a C) -> Self {
        RemovableTripletsOfEdges {
            first_edge_iterator: graph.edges(),
            second_edge_iterator: graph.edges(),
            third_edge_iterator: graph.edges(),
            first_edge_current: None,
            second_edge_current: None,
            graph,
            local_graph: graph.clone(),
            colouriser,
        }
    }
}
