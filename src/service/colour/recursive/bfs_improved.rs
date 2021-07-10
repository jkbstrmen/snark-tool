use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;
use crate::service::colour::colouriser::Colouriser;
use std::collections::VecDeque;

///
/// Colorizer for (sub)cubic graphs only
/// version 2.2
///
#[derive(Debug, Clone)]
pub struct BFSColourizerImproved {}

struct BFSColourizerImprovedGraph {
    // pair - (neighbor, color)
    vertices: Vec<[(usize, usize); 3]>,
    one_edge_vert: Vec<usize>,
    non_colored_edges: Vec<usize>,
    non_colored_edges_of_graph: usize,

    to_visit: VecDeque<usize>,
}

impl Colouriser for BFSColourizerImproved {
    fn is_colorable<G: Graph>(graph: &G) -> bool {
        let mut vertices = Vec::with_capacity(graph.size());
        // create local graph
        for vertex in graph.vertices() {
            let mut neighbors = [(0, 0); 3];
            let mut i = 0;
            for edge in graph.edges_of_vertex(vertex.index()) {
                neighbors[i].1 = 1;
                if edge.from() == vertex.index() {
                    neighbors[i].0 = edge.to();
                } else {
                    neighbors[i].0 = edge.from();
                }
                i += 1;
            }
            vertices.push(neighbors);
        }
        let mut color_graph = BFSColourizerImprovedGraph {
            vertices,
            one_edge_vert: vec![],
            non_colored_edges: vec![],
            non_colored_edges_of_graph: 0,
            to_visit: VecDeque::new(),
        };

        // precolor
        let colors = [3, 4, 5];
        let mut first_vertex = 0;
        let mut index = 0;
        for vertex in color_graph.vertices.clone() {
            // if vertex has only 2 neighbors - skip
            if vertex[2].1 == 0 {
                index += 1;
                continue;
            }

            let mut i = 0;
            for neighbor in vertex.iter() {
                if neighbor.1 == 1 {
                    color_graph.set_edge_color(index, neighbor.0, colors[i]);
                    first_vertex = neighbor.0;
                    color_graph.to_visit.push_back(neighbor.0);
                    i += 1;
                }
            }
            index += 1;
            if i != 0 {
                break;
            }
        }
        let mut non_colored_edges_of_graph = 0;
        let mut non_colored_edges_of_vertex_count = vec![0; graph.size()];
        let mut one_edge_vert = vec![];
        let mut i = 0;
        for vertex in &color_graph.vertices {
            for neighbor in vertex.iter() {
                if neighbor.1 == 1 {
                    non_colored_edges_of_vertex_count[i] += 1;
                    non_colored_edges_of_graph += 1;
                }
            }
            if non_colored_edges_of_vertex_count[i] == 1 {
                if vertex[2].1 != 0 {
                    one_edge_vert.push(i);
                }
            }
            i += 1;
        }
        non_colored_edges_of_graph = non_colored_edges_of_graph / 2;

        color_graph.one_edge_vert = one_edge_vert;
        color_graph.non_colored_edges = non_colored_edges_of_vertex_count;
        color_graph.non_colored_edges_of_graph = non_colored_edges_of_graph;
        first_vertex = color_graph.to_visit.pop_front().unwrap();
        color_graph.color(first_vertex)
    }

    fn new() -> Self {
        BFSColourizerImproved {}
    }
}

impl BFSColourizerImprovedGraph {
    fn color(&mut self, vertex: usize) -> bool {
        let color_vars = [(4, 5), (3, 5), (3, 4)];

        let mut neighbor1 = 0;
        let mut neighbor2 = 0;
        let mut colored_sum: usize = 0;
        for neighbor in self.vertices[vertex].iter() {
            let color = neighbor.1;
            if color == 1 {
                neighbor2 = neighbor1;
                neighbor1 = neighbor.0;
            } else {
                colored_sum += color;
            }
        }

        match self.non_colored_edges[vertex] {
            0 => {
                if self.non_colored_edges_of_graph == 0 {
                    return true;
                } else {
                    // if let Some(next) = self.to_visit.pop_front(){
                    //     return self.color(next);
                    // }
                    while let Some(next) = self.to_visit.pop_front() {
                        if (self.non_colored_edges[next] == 1)
                            || (self.non_colored_edges[next] == 2)
                        {
                            return self.color(next);
                        }
                    }

                    let mut vert = 0;
                    // why skipping vertices with 3 non coloured edges? - algorithm wouldn't handle this scenario
                    while (self.non_colored_edges[vert] != 1) && (self.non_colored_edges[vert] != 2)
                    {
                        vert += 1;
                    }
                    return self.color(vert);
                }
            }
            1 => {
                self.non_colored_edges_of_graph -= 1;
                self.non_colored_edges[vertex as usize] -= 1;
                self.non_colored_edges[neighbor1 as usize] -= 1;
                // let mut change = false;
                let mut from_to_visit = false;
                let mut from_one_vert = false;
                let next_vertex;

                // TODO - not working for subcubic graphs

                if self.non_colored_edges[neighbor1 as usize] == 1 {
                    next_vertex = neighbor1;
                } else if !self.one_edge_vert.is_empty() {
                    next_vertex = self.one_edge_vert.pop().unwrap();
                    self.to_visit.push_back(neighbor1);
                    from_one_vert = true;
                } else {
                    self.to_visit.push_back(neighbor1);
                    next_vertex = self.to_visit.pop_front().unwrap();
                    from_to_visit = true;
                }

                // self.to_visit.push_back(neighbor1);
                // next_vertex = self.to_visit.pop_front().unwrap();

                match colored_sum {
                    0 => {
                        if self.set_edge_check_and_color_rest(neighbor1, vertex, 3, next_vertex) {
                            return true;
                        }
                        if self.set_edge_check_and_color_rest(neighbor1, vertex, 4, next_vertex) {
                            return true;
                        }
                        if self.set_edge_check_and_color_rest(neighbor1, vertex, 5, next_vertex) {
                            return true;
                        }
                    }
                    3 => {
                        if self.set_edge_check_and_color_rest(neighbor1, vertex, 4, next_vertex) {
                            return true;
                        }
                        if self.set_edge_check_and_color_rest(neighbor1, vertex, 5, next_vertex) {
                            return true;
                        }
                    }
                    4 => {
                        if self.set_edge_check_and_color_rest(neighbor1, vertex, 3, next_vertex) {
                            return true;
                        }
                        if self.set_edge_check_and_color_rest(neighbor1, vertex, 5, next_vertex) {
                            return true;
                        }
                    }
                    5 => {
                        if self.set_edge_check_and_color_rest(neighbor1, vertex, 3, next_vertex) {
                            return true;
                        }
                        if self.set_edge_check_and_color_rest(neighbor1, vertex, 4, next_vertex) {
                            return true;
                        }
                    }
                    _ => {
                        // vertex has 3 edges and two of them are colored
                        if self.set_edge_check_and_color_rest(
                            neighbor1,
                            vertex,
                            12 - colored_sum,
                            next_vertex,
                        ) {
                            return true;
                        }
                    }
                }
                // revert changes
                if from_to_visit {
                    self.to_visit.pop_back();
                    self.to_visit.push_front(next_vertex);
                }
                if from_one_vert {
                    self.to_visit.pop_back();
                    self.one_edge_vert.push(next_vertex);
                }
                // self.to_visit.pop_back();
                // self.one_edge_vert.push(next_vertex);

                self.non_colored_edges_of_graph += 1;
                self.non_colored_edges[vertex as usize] += 1;
                self.non_colored_edges[neighbor1 as usize] += 1;
                self.set_edge_color(vertex, neighbor1, 1);
                return false;
            }
            2 => {
                self.non_colored_edges_of_graph -= 2;
                self.non_colored_edges[vertex as usize] -= 2;
                self.non_colored_edges[neighbor1 as usize] -= 1;
                self.non_colored_edges[neighbor2 as usize] -= 1;
                let mut one_edge_neighbors = 0;
                let mut next_from_queue = false;
                let next_vertex;

                // TODO - what if not added to neither one edge neither to visit?

                if self.non_colored_edges[neighbor1 as usize] == 1 {
                    if self.vertices[neighbor1 as usize][2].1 != 0 {
                        one_edge_neighbors += 1;
                        self.one_edge_vert.push(neighbor1);
                    } else {
                        self.to_visit.push_back(neighbor1);
                    }
                } else {
                    self.to_visit.push_back(neighbor1);
                }
                if self.non_colored_edges[neighbor2 as usize] == 1 {
                    if self.vertices[neighbor2 as usize][2].1 != 0 {
                        one_edge_neighbors += 1;
                        self.one_edge_vert.push(neighbor2);
                    } else {
                        self.to_visit.push_back(neighbor2);
                    }
                } else {
                    self.to_visit.push_back(neighbor2);
                }
                if !self.one_edge_vert.is_empty() {
                    next_from_queue = true;
                    next_vertex = self.one_edge_vert.pop().unwrap();
                } else {
                    if let Some(next) = self.to_visit.pop_front() {
                        next_vertex = next;
                    } else {
                        panic!("here");
                    }

                    // next_vertex = self.to_visit.pop_front().unwrap();
                }

                // self.to_visit.push_back(neighbor1);
                // self.to_visit.push_back(neighbor2);
                // next_vertex = self.to_visit.pop_front().unwrap();

                match colored_sum {
                    0 => {
                        for num in 3..6 {
                            self.set_edge_color(vertex, neighbor1, color_vars[num - 3].0);
                            self.set_edge_color(vertex, neighbor2, color_vars[num - 3].1);
                            if self.are_vertices_without_conflict(neighbor1, neighbor2)
                                && self.color(next_vertex)
                            {
                                return true;
                            }

                            self.set_edge_color(vertex, neighbor1, color_vars[num - 3].1);
                            self.set_edge_color(vertex, neighbor2, color_vars[num - 3].0);
                            if self.are_vertices_without_conflict(neighbor1, neighbor2)
                                && self.color(next_vertex)
                            {
                                return true;
                            }
                        }
                    }
                    _ => {
                        self.set_edge_color(vertex, neighbor1, color_vars[colored_sum - 3].0);
                        self.set_edge_color(vertex, neighbor2, color_vars[colored_sum - 3].1);
                        if self.are_vertices_without_conflict(neighbor1, neighbor2)
                            && self.color(next_vertex)
                        {
                            return true;
                        }

                        self.set_edge_color(vertex, neighbor1, color_vars[colored_sum - 3].1);
                        self.set_edge_color(vertex, neighbor2, color_vars[colored_sum - 3].0);
                        if self.are_vertices_without_conflict(neighbor1, neighbor2)
                            && self.color(next_vertex)
                        {
                            return true;
                        }
                    }
                }
                // revert changes
                if one_edge_neighbors == 2 {
                    self.one_edge_vert.pop();
                }
                if one_edge_neighbors == 1 {
                    self.to_visit.pop_back();
                }
                if next_from_queue && one_edge_neighbors == 0 {
                    self.one_edge_vert.push(next_vertex);
                    self.to_visit.pop_back();
                    self.to_visit.pop_back();
                }
                if !next_from_queue && one_edge_neighbors == 0 {
                    self.to_visit.push_front(next_vertex);
                    self.to_visit.pop_back();
                    self.to_visit.pop_back();
                }

                // self.to_visit.push_front(next_vertex);
                // self.to_visit.pop_back();
                // self.to_visit.pop_back();

                self.non_colored_edges_of_graph += 2;
                self.non_colored_edges[vertex as usize] += 2;
                self.non_colored_edges[neighbor1 as usize] += 1;
                self.non_colored_edges[neighbor2 as usize] += 1;
                self.set_edge_color(vertex, neighbor1, 1);
                self.set_edge_color(vertex, neighbor2, 1);
                return false;
            }
            _ => {
                panic!("More than 2 uncolored edges");
            }
        }
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

    // cond1
    fn set_color_and_check_validity(&mut self, from: usize, to: usize, color: usize) -> bool {
        self.set_edge_color(from, to, color);
        self.is_vertex_without_conflict(&self.vertices[from])
    }

    // cond2
    fn are_vertices_without_conflict(&self, first: usize, second: usize) -> bool {
        self.is_vertex_without_conflict(&self.vertices[first])
            && self.is_vertex_without_conflict(&self.vertices[second])
    }

    fn set_edge_check_and_color_rest(
        &mut self,
        from: usize,
        to: usize,
        color: usize,
        next_vertex: usize,
    ) -> bool {
        self.set_color_and_check_validity(from, to, color) && self.color(next_vertex)
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
