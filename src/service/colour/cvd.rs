use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;

use rand::seq::SliceRandom;
use rand::Rng;

struct CVDGraph {
    // vertices: Vec<[(usize, u8); 3]>,
    vertices: Vec<Vec<(usize, u8)>>,
    // available_colors: Vec<Vec<u8>>,
    size: usize,

    vertices_to_try: Vec<usize>,
    conflicting_vertices: Vec<usize>,

    kempe_chain: KempeChain,
}

struct KempeChain {
    vertices: Vec<usize>,
    conflicting_colour: u8,
    resolving_colour: u8,
    last_colour_of_chain: u8,
}

pub fn is_colorable<G>(graph: &G) -> Option<bool>
where
    G: Graph,
{
    let mut graph = create_cvd_graph(graph);
    let l_limit = graph.vertices_to_try.len();

    for i in 0..l_limit {
        graph.next_pre_colour();

        let colorable = graph.is_colorable();
        if colorable {
            return Some(true);
        }
    }
    None
}

fn create_cvd_graph<G>(graph: &G) -> CVDGraph
where
    G: Graph,
{
    // let mut vertices: Vec<[(usize, u8); 3]> = Vec::with_capacity(graph.size());
    let mut vertices: Vec<Vec<(usize, u8)>> = Vec::with_capacity(graph.size());
    for vertex in graph.vertices() {
        // let mut vertex_new: [(usize, u8); 3] = [(0, 0); 3];
        let mut vertex_new = vec![];
        let mut i = 0;
        for edge in graph.edges_of_vertex(vertex.index()) {
            if edge.from() == vertex.index() {
                // vertex_new[i] = (edge.to(), 1);
                vertex_new.push((edge.to(), 0));
            } else {
                // vertex_new[i] = (edge.from(), 1);
                vertex_new.push((edge.from(), 0));
            }
            i += 1;
        }
        vertices.push(vertex_new);
    }
    // let colors = vec![3, 4, 5];
    // let mut available = Vec::<Vec<u8>>::with_capacity(graph.size());
    // for _i in 0..graph.size() {
    //     available.push(colors.clone());
    // }
    let mut vertices_to_try = Vec::with_capacity(graph.size());
    for i in 0..graph.size() {
        if graph.edges_of_vertex(i).next().is_some() {
            vertices_to_try.push(i);
        }
    }

    CVDGraph {
        vertices,
        // available_colors: available,
        size: graph.size(),
        vertices_to_try,
        conflicting_vertices: vec![],
        kempe_chain: KempeChain {
            vertices: vec![],
            conflicting_colour: 0,
            resolving_colour: 0,
            last_colour_of_chain: 0,
        },
    }
}

impl CVDGraph {
    fn is_colorable(&mut self) -> bool {
        // todo - how to choose r_limit?
        let r_limit = self.vertices_to_try.len() / 4;

        let mut i = 0;
        while i < r_limit {
            let cvs = self.conflicting_vertices();
            if cvs.is_empty() {
                return true;
            }

            // do not choose randomly twice the same cv
            let next = cvs.choose(&mut rand::thread_rng()).unwrap();

            self.kempe_chain_swap(next.clone());

            let cvs_after = self.conflicting_vertices();
            if cvs.len() == cvs_after.len() {
                i += 1;
            }
        }
        false
    }

    fn next_pre_colour(&mut self) {
        // clean graph colors
        for mut vertex in self.vertices.iter_mut() {
            for neighbor in vertex.iter_mut() {
                // if neighbor.1 != 0 {
                //     neighbor.1 = 1;
                // }
                neighbor.1 = 0;
            }
        }
        let mut to_visit = Vec::with_capacity(self.size);
        let first_vertex = self.next_random_vertex();
        to_visit.push(first_vertex);
        self.bfs_pre_colour(to_visit);
    }

    fn bfs_pre_colour(&mut self, mut to_visit: Vec<usize>) {
        if to_visit.is_empty() {
            return;
        }
        let current = to_visit.pop().unwrap();

        let mut available = self.available_colors_of_vertex(current);
        let uncolored_neighbors_of_vertex = self.uncolored_edges_of_vertex(current);
        if uncolored_neighbors_of_vertex.is_empty() {
            self.bfs_pre_colour(to_visit);
            return;
        }
        // let mut neighbors = self.vertices[current];
        let mut neighbors = self.vertices[current].clone();
        for neighbor in neighbors.iter() {
            // for neighbor in self.vertices[current].iter_mut() {
            if neighbor.1 == 0 {
                let color = available.pop().unwrap();
                // neighbor.1 = color;
                self.set_edge_color(current, neighbor.0, color);
                // do not push duplicates?
                to_visit.push(neighbor.0);
            }
        }
        self.bfs_pre_colour(to_visit);
    }

    fn set_edge_color(&mut self, from: usize, to: usize, color: u8) {
        for neighbor in self.vertices[from].iter_mut() {
            if neighbor.0 == to {
                neighbor.1 = color;
            }
        }
        for neighbor in self.vertices[to].iter_mut() {
            if neighbor.0 == from {
                neighbor.1 = color;
            }
        }
    }

    fn available_colors_of_vertex(&self, vertex: usize) -> Vec<u8> {
        let mut colors = vec![3, 4, 5];
        for neighbor in self.vertices[vertex].iter() {
            colors.retain(|&item| item != neighbor.1)
        }
        colors
    }

    fn uncolored_edges_of_vertex(&self, vertex: usize) -> Vec<usize> {
        let mut uncolored_neighbors = vec![];
        for neighbor in self.vertices[vertex].iter() {
            if neighbor.1 == 0 {
                uncolored_neighbors.push(neighbor.0);
            }
        }
        uncolored_neighbors
    }

    fn conflicting_color_of_vertex(&self, vertex: usize) -> Option<u8> {
        let mut index = 0;
        let vertex = &self.vertices[vertex];
        for neighbor in vertex.iter() {
            let next_index = (index + 1) % vertex.len();
            if vertex[index].1 == vertex[next_index].1 {
                return Some(neighbor.1);
            }
            index += 1;
        }
        None
    }

    fn next_random_vertex(&mut self) -> usize {
        let next = self
            .vertices_to_try
            .choose(&mut rand::thread_rng())
            .unwrap();
        let to_return = next.clone();
        self.vertices_to_try.retain(|&x| x != to_return);
        to_return
    }

    fn conflicting_vertices(&self) -> Vec<usize> {
        let mut cvs = vec![];
        let mut index = 0;
        for vertex in self.vertices.iter() {
            if self.is_conflicting_vertex(vertex) {
                cvs.push(index);
            }
            index += 1;
        }
        cvs
    }

    fn kempe_chain_swap(&mut self, start_vertex: usize) {
        self.kempe_chain.vertices = vec![];
        self.kempe_chain.resolving_colour = 0;
        self.kempe_chain.conflicting_colour = 0;
        self.kempe_chain.last_colour_of_chain = 0;

        self.kempe_chain.vertices.push(start_vertex);

        let conflicting_colour = self.conflicting_color_of_vertex(start_vertex).unwrap();
        let resolving_colour = self.available_colors_of_vertex(start_vertex)[0];
        self.kempe_chain.conflicting_colour = conflicting_colour;
        self.kempe_chain.resolving_colour = resolving_colour;

        let mut next = None;
        for neighbors in self.vertices[start_vertex].iter() {
            if neighbors.1 == self.kempe_chain.conflicting_colour {
                next = Some(neighbors.0);
            }
        }

        self.kempe_step(next.unwrap(), resolving_colour);
    }

    fn next_vertex_of_chain(&self) -> Option<(usize, u8)> {
        let mut next = None;
        let mut next_color = 0;
        let last_vertex_of_chain = self.kempe_chain.vertices[self.kempe_chain.vertices.len() - 1];
        let second_last_vertex_of_chain =
            self.kempe_chain.vertices[self.kempe_chain.vertices.len() - 2];
        for neighbor in self.vertices[last_vertex_of_chain].iter() {
            if neighbor.1 == self.kempe_chain.last_colour_of_chain
                && neighbor.0 != second_last_vertex_of_chain
            {
                next = Some(neighbor.0);
            }
        }
        // for subcubic graphs
        if next.is_none() {
            return None;
        }
        if self.kempe_chain.last_colour_of_chain == self.kempe_chain.conflicting_colour {
            next_color = self.kempe_chain.resolving_colour;
        } else {
            next_color = self.kempe_chain.conflicting_colour;
        }
        Some((next.unwrap(), next_color))
    }

    fn kempe_step(
        &mut self,
        next_vertex: usize,
        next_colour: u8, /*, previous: usize, current: usize, new_color: u8*/
    ) {
        if self.kempe_chain.vertices.len() > self.size {
            return;
        }

        let current = self.kempe_chain.vertices[self.kempe_chain.vertices.len() - 1];

        if self.is_conflicting_vertex(&self.vertices[next_vertex]) {
            self.set_edge_color(current, next_vertex, next_colour);
            return;
        }
        self.kempe_chain.vertices.push(next_vertex);
        self.kempe_chain.last_colour_of_chain = next_colour;
        self.set_edge_color(current, next_vertex, next_colour);

        // get next
        let next = self.next_vertex_of_chain();
        // adjustment for subcubic graphs - if none - conflict is cancelled out
        if next.is_none() {
            return;
        }
        let next = next.unwrap();
        self.kempe_step(next.0, next.1);
    }

    // fn is_conflicting_vertex(&self, vertex: &[(usize, u8); 3]) -> bool {
    fn is_conflicting_vertex(&self, vertex: &Vec<(usize, u8)>) -> bool {
        let mut colours = vec![];
        for neighbor in vertex.iter() {
            colours.push(neighbor.1);
        }
        colours.sort();
        let size_before = colours.len();
        colours.dedup();
        if size_before > colours.len() {
            return true;
        }
        false
    }

    // temp
    // fn print(&self) {
    //     let mut i = 0;
    //     for vertex in self.vertices.iter() {
    //         print!("{}: ", i);
    //         println!("{:?}", vertex);
    //         i += 1;
    //     }
    //     println!("\n");
    // }
}
