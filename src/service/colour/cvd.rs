use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;

// extern crate rand;

use rand::Rng;
use rand::seq::SliceRandom;

// pub struct CVDColourizer<G, V, E>
// where
//     G: Graph<V, E>,
//     V: Vertex,
//     E: Edge, {
//
//
// }

struct CVDGraph {
    vertices: Vec<[(usize, u8); 3]>,
    // available_colors: Vec<Vec<u8>>,
    size: usize,

    vertices_to_try: Vec<usize>,
    conflicting_vertices: Vec<usize>,

    kempe_chain: Vec<usize>
}


pub fn is_colorable<G, V, E>(graph: &G) -> Option<bool>
where
    G: Graph<V, E>,
    V: Vertex,
    E: Edge,
{
    let mut graph = create_cvd_graph(graph);

    let l_limit = 5;
    for i in 0..l_limit {
        // iteration
        graph.next_pre_colour();
        let colorable = graph.is_colorable();
        if colorable {
            return Some(true);
        }
    }

    None
}

fn create_cvd_graph<G, V, E>(graph: &G) -> CVDGraph
where
    G: Graph<V, E>,
    V: Vertex,
    E: Edge,
{
    let mut vertices: Vec<[(usize, u8); 3]> = Vec::with_capacity(graph.size());
    for vertex in graph.vertices() {
        let mut vertex_new: [(usize, u8); 3] = [(0, 0); 3];
        let mut i = 0;
        for edge in graph.edges_of_vertex(vertex.index()) {
            if edge.from() == vertex.index() {
                vertex_new[i] = (edge.to(), 0);
            } else {
                vertex_new[i] = (edge.from(), 0);
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
        vertices_to_try.push(i);
    }

    CVDGraph {
        vertices,
        // available_colors: available,
        size: graph.size(),
        vertices_to_try,
        conflicting_vertices: vec![],
        kempe_chain: vec![]
    }
}

impl CVDGraph {
    fn is_colorable(&mut self) -> bool {
        let r_limit = 10;
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
        if available.is_empty() {
            return;
        }
        let mut neighbors = self.vertices[current];
        for neighbor in neighbors.iter_mut() {
            if neighbor.1 == 0 {
                let color = available.pop().unwrap();
                neighbor.1 = color;
                self.set_edge_color(current, neighbor.0, color);
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
            colors.retain(|&item| { item != neighbor.1 })
        }
        colors
    }

    fn next_random_vertex(&mut self) -> usize {
        let next = self.vertices_to_try.choose(&mut rand::thread_rng()).unwrap();
        let to_return = next.clone();
        self.vertices_to_try.retain(|&x| x != to_return);
        to_return
    }

    fn conflicting_vertices(&self) -> Vec<usize> {
        let mut cvs = vec![];

        let mut index = 0;
        for vertex in self.vertices.iter() {
            let mut colours = vec![];
            for neighbor in vertex.iter() {
                colours.push(neighbor.1);
            }
            colours.sort();
            let size_before = colours.len();
            colours.dedup();
            if size_before > colours.len() {
                cvs.push(index);
            }
            index += 1;
        }
        cvs
    }

    fn kempe_chain_swap(&mut self, start_vertex: usize){
        self.kempe_chain.push(start_vertex);

        // TODO

        // find edge to go to
        let next = self.next_vertex_of_chain(start_vertex, start_vertex);

        // resolve second color

        // go to vertex by edge


    }

    fn next_vertex_of_chain(&self, current: usize, previous: usize) -> (usize, u8) {
        let vertex = &self.vertices[current];
        let mut next_vertex = 0;
        let mut conflicting_colour;
        let mut index = 0;
        for neighbor in vertex.iter() {
            if neighbor.0 == previous {
                index += 1;
                continue;
            }

            let next_index = (index+1)% vertex.len();
            if vertex[index].1 == vertex[next_index].1 {
                next_vertex = neighbor.0;
                conflicting_colour = neighbor.1;
            }
            index += 1;
        }

        let available = self.available_colors_of_vertex(current);
        let resolving_colour = available[0];
        (next_vertex, resolving_colour)
    }

    fn kempe_step(&mut self, previous: usize, current: usize, new_color: u8){
        if self.kempe_chain.len() == self.size { return; }

        if self.is_conflicting_vertex(current) {
           // swap edge color
            self.set_edge_color(previous, current, new_color);
            return;
        }
        self.set_edge_color(previous, current, new_color);

        // find edge to go to
        // self.kempe_step(current, next, ...);

    }

    fn is_conflicting_vertex(&self, vertex: usize) -> bool {

        // TODO

        false
    }

    // fn swap_colours_of_edges(&mut self, ){
    //
    // }
}