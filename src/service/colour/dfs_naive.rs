use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::service::colour::colouriser::Colourizer;
use std::collections;
use std::collections::VecDeque;
use crate::graph::vertex::Vertex;

pub struct DFSColourizerNaive {}

struct BFSColourizerGraph {
    // pair - (neighbor, color)
    vertices: Vec<[(usize, usize); 3]>,
    // vertices: Vec<Vec<(usize, usize)>>,
    one_edge_vert: Vec<usize>,
    non_colored_edges: Vec<usize>,
    non_colored_edges_of_graph: usize,

    queue: collections::VecDeque<usize>
}

impl Colourizer for DFSColourizerNaive {
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
        let mut color_graph = BFSColourizerGraph {
            vertices,
            one_edge_vert: vec![],
            non_colored_edges: vec![],
            non_colored_edges_of_graph: 0,
            queue: collections::VecDeque::new()
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
                    i += 1;
                }
            }
            index += 1;
            if i != 0 {
                break;
            }
        }

        color_graph.queue.push_back(first_vertex);

        unsafe {
            color_graph.color_recursive()
        }
    }

    // fn is_colorable<G: Graph>(graph: &G) -> bool {
    //     // create local graph
    //     let graph_matrix = vec![0; graph.size() * graph.size()];
    //     let mut colour_graph = BFSColourizerGraph2 {
    //         graph: graph_matrix,
    //         neighbors: vec![[0, 0, 0]; graph.size()],
    //         one_edge_vert: vec![],
    //         non_colored_edges_count: vec![0; graph.size()],
    //         non_colored_edges_of_graph: 0,
    //         graph_size: graph.size(),
    //     };
    //     for edge in graph.edges() {
    //         colour_graph.set_edge_color(edge.from(), edge.to(), 1);
    //     }
    //
    //     // precolor first vertex
    //     let mut first_vertex = 0;
    //     let colours = [3, 4, 5];
    //     let mut vertex_coloured = false;
    //     let mut vertex = 0;
    //     let mut colour_position = 0;
    //     while !vertex_coloured {
    //         for neighbor in 0..graph.size() {
    //             if colour_graph.get_edge_color(vertex, neighbor) == 1 {
    //                 colour_graph.set_edge_color(vertex, neighbor, colours[colour_position]);
    //                 colour_position += 1;
    //                 first_vertex = vertex;
    //                 vertex_coloured = true;
    //             }
    //         }
    //         vertex += 1;
    //     }
    //     colour_graph.non_colored_edges_of_graph = 0;
    //
    //     // get count of non colored edges for whole graph and for vertices, init neighbors
    //     for vertex in 0..graph.size() {
    //         let mut neighbor_position = 0;
    //         colour_graph.non_colored_edges_count[vertex] = 0;
    //         for neighbor in 0..graph.size() {
    //             if colour_graph.get_edge_color(vertex, neighbor) > 0 {
    //                 colour_graph.neighbors[vertex][neighbor_position] = neighbor;
    //                 neighbor_position += 1;
    //                 if colour_graph.get_edge_color(vertex, neighbor) == 1 {
    //                     colour_graph.non_colored_edges_of_graph += 1;
    //                     colour_graph.non_colored_edges_count[vertex] += 1;
    //                 }
    //             }
    //         }
    //         if neighbor_position < 3 {
    //             colour_graph.neighbors[vertex][neighbor_position] = vertex;
    //         }
    //         neighbor_position += 1;
    //         if neighbor_position < 3 {
    //             colour_graph.neighbors[vertex][neighbor_position] = vertex;
    //         }
    //     }
    //
    //     colour_graph.non_colored_edges_of_graph = colour_graph.non_colored_edges_of_graph / 2;
    //     for vertex in 0..graph.size() {
    //         if colour_graph.non_colored_edges_count[vertex] == 1 {
    //             colour_graph.one_edge_vert.push(vertex);
    //         }
    //     }
    //     colour_graph.color(first_vertex)
    // }

    fn new() -> Self {
        Self {}
    }
}

impl DFSColourizerNaive {}

impl BFSColourizerGraph {


    unsafe fn color_recursive(&mut self) -> bool {
        if self.queue.is_empty() {
            return true;
        }
        let vertex = self.queue.pop_back().unwrap();
        let available_colors = self.available_colors_of_vertex(vertex);
        if available_colors.len() == 0 {
            for index in 0..self.vertices.len() {
                let av = self.available_colors_of_vertex(index).len();
                if av > 0 && av < 3 {
                    self.queue.push_back(index);
                    if self.color_recursive() {
                        return true;
                    }
                    self.queue.pop_back();
                }
            }
            return false;
        }
        let self_ref = self as *const BFSColourizerGraph;
        if available_colors.len() == 1 {
            for neighbor in (*self_ref).vertices[vertex].iter() {
                if neighbor.1 == 1 {
                    self.set_edge_color(vertex, neighbor.0, available_colors[0]);
                    let without_conflict = self.is_vertex_without_conflict(&self.vertices[neighbor.0]);
                    if without_conflict {
                        self.queue.push_back(neighbor.0);
                        if self.color_recursive() {
                            return true;
                        }
                        self.queue.pop_back();
                    }
                    self.set_edge_color(vertex, neighbor.0, 1);
                }
            }
            // panic!("we could not find non coloured vertex, even if we have available colour");
        }
        if available_colors.len() == 2 {
            let mut available_colours_variations = vec![];
            available_colours_variations.push(vec![available_colors[0], available_colors[1]]);
            available_colours_variations.push(vec![available_colors[1], available_colors[0]]);
            for available_colours_variation in available_colours_variations {

                let mut first_neighbor = 0;
                let mut second_neighbor = 0;
                for neighbor in (*self_ref).vertices[vertex].iter() {
                    if neighbor.1 == 1 {
                        first_neighbor = second_neighbor;
                        second_neighbor = neighbor.0;
                    }
                }

                self.set_edge_color(vertex, first_neighbor, available_colours_variation[0]);
                let without_conflict = self.is_vertex_without_conflict(&self.vertices[first_neighbor]);
                if !without_conflict {
                    self.set_edge_color(vertex, first_neighbor, 1);
                    continue;
                }
                self.set_edge_color(vertex, second_neighbor, available_colours_variation[1]);
                let without_conflict = self.is_vertex_without_conflict(&self.vertices[second_neighbor]);
                if !without_conflict {
                    self.set_edge_color(vertex, first_neighbor, 1);
                    continue;
                }
                self.queue.push_back(first_neighbor);
                // self.queue.push_back(second_neighbor);

                if self.color_recursive() {
                    return true;
                }

                // pop_back??
                self.queue.pop_back();
                // self.queue.pop_back();

                self.set_edge_color(vertex, first_neighbor, 1);
                self.set_edge_color(vertex, second_neighbor, 1);
            }
        }

        self.queue.push_back(vertex);
        false
    }

    fn available_colors_of_vertex(&self, vertex: usize) -> Vec<usize> {
        let mut colors = vec![3, 4, 5];
        for neighbor in self.vertices[vertex].iter() {
            colors.retain(|&item| item != neighbor.1)
        }
        colors
    }




    // ============================================================
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
                let mut change = false;
                let next_vertex;

                if self.non_colored_edges[neighbor1 as usize] == 1 {
                    next_vertex = neighbor1;
                } else if !self.one_edge_vert.is_empty() {
                    change = true;
                    next_vertex = self.one_edge_vert.pop().unwrap();
                } else {
                    next_vertex = neighbor1;
                }

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
                if change {
                    self.one_edge_vert.push(next_vertex);
                }
                // revert changes
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

                if self.non_colored_edges[neighbor1 as usize] == 1 {
                    if self.vertices[neighbor1 as usize][2].1 != 0 {
                        one_edge_neighbors += 1;
                        self.one_edge_vert.push(neighbor1);
                    }
                }
                if self.non_colored_edges[neighbor2 as usize] == 1 {
                    if self.vertices[neighbor2 as usize][2].1 != 0 {
                        one_edge_neighbors += 1;
                        self.one_edge_vert.push(neighbor2);
                    }
                }
                if !self.one_edge_vert.is_empty() {
                    next_from_queue = true;
                    next_vertex = self.one_edge_vert.pop().unwrap();
                } else {
                    next_vertex = neighbor1;
                }

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
                if next_from_queue && one_edge_neighbors == 0 {
                    self.one_edge_vert.push(next_vertex);
                }
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
