use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;
use crate::service::chromatic_properties::resistance::Resistance;
use crate::service::colour::colouriser::Colouriser;
use std::collections::HashMap;

pub struct Resistibility<G, C>
where
    G: Graph + Clone,
    C: Colouriser,
{
    graph: G,
    _colourizer: C,
    edges_resistibilities: HashMap<(usize, usize), usize>,
    vertex_resistibilities: Vec<Option<usize>>,
}

impl<G, C> Resistibility<G, C>
where
    G: Graph + Clone,
    C: Colouriser,
{
    pub fn of_graph_with_colouriser(graph: &G, colourizer: C) -> Self {
        Resistibility {
            graph: graph.clone(),
            _colourizer: colourizer,
            edges_resistibilities: HashMap::new(),
            vertex_resistibilities: vec![None; graph.size()],
        }
    }

    pub fn edges_resistibility(&mut self) -> &HashMap<(usize, usize), usize> {
        let mut local_graph = self.graph.clone();
        for edge in self.graph.edges() {
            if self
                .edges_resistibilities
                .get(&(edge.from(), edge.to()))
                .is_none()
            {
                let edge_resistibility =
                    self.edge_resistibility(&mut local_graph, edge.from(), edge.to());
                self.edges_resistibilities
                    .insert((edge.from(), edge.to()), edge_resistibility);
            }
        }
        &self.edges_resistibilities
    }

    #[allow(dead_code)]
    pub fn take_edges_resistibility(&mut self) -> HashMap<(usize, usize), usize> {
        self.edges_resistibility();
        std::mem::replace(&mut self.edges_resistibilities, HashMap::new())
    }

    pub fn edge_resistibility(&self, graph: &mut G, from: usize, to: usize) -> usize {
        graph.remove_edge(from, to);
        let resistance = Resistance::new_with_colouriser(C::new());
        let resistance = resistance.edge_resistance(graph);
        graph.add_edge(from, to);
        resistance.unwrap() + 1
    }

    pub fn edge_resistibility_index(&mut self) -> usize {
        self.edges_resistibility();
        let resistance = Resistance::new_with_colouriser(C::new());
        let resistance = resistance.edge_resistance(&self.graph).unwrap();
        let mut index = 0;
        for edges_resistibility in self.edges_resistibilities.iter() {
            if edges_resistibility.1 > &resistance {
                index += 1;
            }
        }
        index
    }

    pub fn vertices_resistibility(&mut self) -> Vec<usize> {
        let mut local_graph = self.graph.clone();
        let mut v_resistibilities = vec![0; self.graph.size()];
        for vertex in self.graph.vertices() {
            if self.vertex_resistibilities[vertex.index()].is_none() {
                let vertex_resistibility =
                    self.vertex_resistibility(&mut local_graph, vertex.index());
                self.vertex_resistibilities[vertex.index()] = Some(vertex_resistibility);
            }
            v_resistibilities[vertex.index()] =
                self.vertex_resistibilities[vertex.index()].unwrap();
        }
        v_resistibilities
    }

    pub fn vertex_resistibility(&self, graph: &mut G, vertex: usize) -> usize {
        let edges = graph.edges_of_vertex(vertex);
        let mut backup_edges = vec![];
        for edge in edges {
            backup_edges.push((edge.from(), edge.to()));
        }
        graph.remove_edges_of_vertex(vertex);
        let resistance = Resistance::new_with_colouriser(C::new());
        let resistance = resistance.vertex_resistance(graph);
        for backup_edge in backup_edges {
            graph.add_edge(backup_edge.0, backup_edge.1);
        }
        resistance.unwrap() + 1
    }

    pub fn vertex_resistibility_index(&mut self) -> usize {
        let resistibilities = self.vertices_resistibility();
        let resistance = Resistance::new_with_colouriser(C::new());
        let resistance = resistance.vertex_resistance(&self.graph).unwrap();
        let mut index = 0;
        for resistibility in resistibilities {
            if resistibility > resistance {
                index += 1;
            }
        }
        index
    }

    #[allow(dead_code)]
    pub fn edges_resistibility_parallel(&mut self) -> HashMap<(usize, usize), usize> {
        // TODO

        HashMap::new()
    }

    #[allow(dead_code)]
    pub fn vertices_resistibility_parallel(&mut self) -> Vec<usize> {
        // TODO

        vec![]
    }
}
