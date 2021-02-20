use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;
use crate::service::colour::colouriser::Colouriser;
use crate::service::graph_traversal::bfs_temp::BfsOfGraph;
use crate::service::graph_traversal::dfs::DfsOfGraph;
use serde::export::Option::Some;

use std::time::Instant;
pub static mut ELAPSED: u128 = 0;

pub struct BFSColouriserBasic {}

#[derive(Debug, Clone)]
struct BFSColouriserGraph<'a, G: Graph + Clone> {
    vertices: Vec<[(usize, usize); 3]>,
    bfs: BfsOfGraph<'a, G>,
}

impl BFSColouriserBasic {
    pub fn is_colorable<G: Graph + Clone>(graph: &G) -> bool {
        let mut color_graph = BFSColouriserGraph::new(graph);
        color_graph.color()
    }

    fn new() -> Self {
        BFSColouriserBasic {}
    }
}

impl<'a, G: Graph + Clone> BFSColouriserGraph<'a, G> {
    pub fn new(graph: &'a G) -> Self {
        let mut vertices = Vec::with_capacity(graph.size());
        // create local graph
        for vertex in graph.vertices() {
            let mut neighbors = [(0, 0); 3];
            let mut i = 0;
            for edge in graph.edges_of_vertex(vertex.index()) {
                // important for subcubic graphs?
                // neighbors[i].1 = 1;
                neighbors[i].1 = 0;
                if edge.from() == vertex.index() {
                    neighbors[i].0 = edge.to();
                } else {
                    neighbors[i].0 = edge.from();
                }
                i += 1;
            }
            vertices.push(neighbors);
        }
        let first_vertex = 0;
        let colors = vec![3, 4, 5];
        let bfs = BfsOfGraph::new(graph, first_vertex);

        let mut color_graph = BFSColouriserGraph { vertices, bfs };

        // pre color edges of first vertex
        color_graph.bfs.next();
        color_graph.set_edge_color(
            first_vertex,
            color_graph.vertices[first_vertex][0].0,
            colors[0],
        );
        color_graph.set_edge_color(
            first_vertex,
            color_graph.vertices[first_vertex][1].0,
            colors[1],
        );
        color_graph.set_edge_color(
            first_vertex,
            color_graph.vertices[first_vertex][2].0,
            colors[2],
        );

        color_graph
    }

    fn color(&mut self) -> bool {
        // let local_copy = self.clone();
        // let color_vars = [(4, 5), (3, 5), (3, 4)];

        // TODO
        // remember color sum of each vertex - probably wont help

        // improve performance of BFS somehow?

        let begin = Instant::now();

        let local_copy = self.clone();
        let next = self.bfs.next();

        unsafe {
            ELAPSED += begin.elapsed().as_nanos();
        }

        // if let Some(next) = self.bfs.next() {
        if let Some(next) = next {
            let actual = next.index();
            // println!("{}", actual);

            // compute actual sum of colors of edges of next
            let mut color_sum = 0;
            let mut first_neighbor = 0;
            let mut second_neighbor = 0;
            for neighbor in self.vertices[next.index()].iter() {
                color_sum += neighbor.1;

                if neighbor.1 == 0 {
                    second_neighbor = first_neighbor;
                    first_neighbor = neighbor.0;
                }
            }

            match color_sum {
                // one already colored edge
                3 => {
                    self.set_edge_color(actual, first_neighbor, 4);
                    self.set_edge_color(actual, second_neighbor, 5);
                    if self.are_vertices_without_conflict(first_neighbor, second_neighbor) {
                        if self.color() {
                            return true;
                        }
                    }
                    self.set_edge_color(actual, first_neighbor, 5);
                    self.set_edge_color(actual, second_neighbor, 4);
                    if self.are_vertices_without_conflict(first_neighbor, second_neighbor) {
                        if self.color() {
                            return true;
                        }
                    }
                    self.set_edge_color(actual, first_neighbor, 0);
                    self.set_edge_color(actual, second_neighbor, 0);
                }
                4 => {
                    self.set_edge_color(actual, first_neighbor, 3);
                    self.set_edge_color(actual, second_neighbor, 5);
                    if self.are_vertices_without_conflict(first_neighbor, second_neighbor) {
                        if self.color() {
                            return true;
                        }
                    }
                    self.set_edge_color(actual, first_neighbor, 5);
                    self.set_edge_color(actual, second_neighbor, 3);
                    if self.are_vertices_without_conflict(first_neighbor, second_neighbor) {
                        if self.color() {
                            return true;
                        }
                    }
                    self.set_edge_color(actual, first_neighbor, 0);
                    self.set_edge_color(actual, second_neighbor, 0);
                }
                5 => {
                    self.set_edge_color(actual, first_neighbor, 3);
                    self.set_edge_color(actual, second_neighbor, 4);
                    if self.are_vertices_without_conflict(first_neighbor, second_neighbor) {
                        if self.color() {
                            return true;
                        }
                    }
                    self.set_edge_color(actual, first_neighbor, 4);
                    self.set_edge_color(actual, second_neighbor, 3);
                    if self.are_vertices_without_conflict(first_neighbor, second_neighbor) {
                        if self.color() {
                            return true;
                        }
                    }
                    self.set_edge_color(actual, first_neighbor, 0);
                    self.set_edge_color(actual, second_neighbor, 0);
                }
                // two already colored edges
                7 => {
                    self.set_edge_color(actual, first_neighbor, 5);
                    if self.is_vertex_without_conflict(&self.vertices[first_neighbor]) {
                        if self.color() {
                            return true;
                        }
                    }
                    self.set_edge_color(actual, first_neighbor, 0);
                }
                8 => {
                    self.set_edge_color(actual, first_neighbor, 4);
                    if self.is_vertex_without_conflict(&self.vertices[first_neighbor]) {
                        if self.color() {
                            return true;
                        }
                    }
                    self.set_edge_color(actual, first_neighbor, 0);
                }
                9 => {
                    self.set_edge_color(actual, first_neighbor, 3);
                    if self.is_vertex_without_conflict(&self.vertices[first_neighbor]) {
                        if self.color() {
                            return true;
                        }
                    }
                    self.set_edge_color(actual, first_neighbor, 0);
                }

                12 => {
                    if self.color() {
                        return true;
                    }
                }
                _ => panic!("unknown color sum: {}", color_sum),
            }

            let begin = Instant::now();

            // self.bfs.back();

            self.bfs = local_copy.bfs;
            self.vertices = local_copy.vertices;

            unsafe {
                ELAPSED += begin.elapsed().as_nanos();
            }

            // self.bfs = local_copy.bfs;
            // self.vertices = local_copy.vertices;
            return false;
        }
        true
    }

    fn set_edge_color(&mut self, from: usize, to: usize, color: usize) {
        for neighbor in self.vertices[from].iter_mut() {
            if neighbor.0 == to {
                neighbor.1 = color;
                break;
            }
        }
        for neighbor in self.vertices[to].iter_mut() {
            if neighbor.0 == from {
                neighbor.1 = color;
                break;
            }
        }
    }

    // ttt2
    fn is_vertex_without_conflict(&self, neighbors: &[(usize, usize); 3]) -> bool {
        is_without_conflict(neighbors[0].1, neighbors[1].1, neighbors[2].1)
    }

    fn are_vertices_without_conflict(&self, first: usize, second: usize) -> bool {
        self.is_vertex_without_conflict(&self.vertices[first])
            && self.is_vertex_without_conflict(&self.vertices[second])
    }
}

fn is_without_conflict(color1: usize, color2: usize, color3: usize) -> bool {
    // ttt
    if (color1 == color2) && (color1 > 1) {
        return false;
    }
    if (color1 == color3) && (color1 > 1) {
        return false;
    }
    if (color3 == color2) && (color3 > 1) {
        return false;
    }
    return true;
}
