use crate::graph::traits::edge::Edge;
use crate::graph::traits::graph::Graph;
use crate::graph::traits::vertex::Vertex;

// Colorizer for (sub)cubic graphs only
pub struct BFSColorizer {
    // pair - (neighbor, color)
    vertices: Vec<[(usize, usize); 3]>,
    one_edge_vert: Vec<usize>,
    non_colored_edges: Vec<usize>,
    non_colored_edges_of_graph: usize,
}

impl BFSColorizer {
    pub fn is_colorable<G, V, E>(graph: &G) -> bool
    where
        G: Graph<V, E>,
        V: Vertex,
        E: Edge,
    {
        let mut vertices = Vec::with_capacity(graph.size());
        for vertex in graph.vertices() {
            let mut neighbors = [(0, 1); 3];
            let mut i = 0;
            for edge in graph.edges_of_vertex(vertex.index()) {
                if edge.from() == vertex.index() {
                    neighbors[i].0 = edge.to();
                } else {
                    neighbors[i].0 = edge.from();
                }
                i += 1;
            }
            // if vertex has less than 3 edges - where is checked later?
            if i < 3 {
                neighbors[i].0 = vertex.index();
            }
            i += 1;
            if i < 3 {
                neighbors[i].0 = vertex.index();
            }
            vertices.push(neighbors);
        }
        let mut color_graph = BFSColorizer {
            vertices,
            one_edge_vert: vec![],
            non_colored_edges: vec![],
            non_colored_edges_of_graph: 0,
        };

        // precolor
        let colors = [3, 4, 5];
        let mut first_vertex = 0;
        let mut index = 0;
        for vertex in color_graph.vertices.clone() {
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
                one_edge_vert.push(i);
            }
            i += 1;
        }
        non_colored_edges_of_graph = non_colored_edges_of_graph / 2;

        color_graph.one_edge_vert = one_edge_vert;
        color_graph.non_colored_edges = non_colored_edges_of_vertex_count;
        color_graph.non_colored_edges_of_graph = non_colored_edges_of_graph;
        color_graph.color(first_vertex)
    }

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
                // println!("zero uncolored edges of vertex {}", vertex);

                if self.non_colored_edges_of_graph == 0 {
                    return true;
                } else {
                    let mut vert = 0;
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
                if self.non_colored_edges[neighbor1 as usize] == 1 && !self.one_edge_vert.is_empty()
                {
                    change = true;
                    next_vertex = self.one_edge_vert.pop().unwrap(); // preco sa ale sem potom do jenodhrannych neprida kde1, ked aj kde1 uz bude jednohran?
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

                // println!("one uncolored edge of vertex {}", vertex);
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
                    one_edge_neighbors += 1;
                    self.one_edge_vert.push(neighbor1);
                }
                if self.non_colored_edges[neighbor2 as usize] == 1 {
                    one_edge_neighbors += 1;
                    self.one_edge_vert.push(neighbor2);
                }
                if !self.one_edge_vert.is_empty() {
                    next_from_queue = true;
                    next_vertex = self.one_edge_vert.pop().unwrap();
                } else {
                    next_vertex = neighbor1;
                }

                match colored_sum {
                    0 => {
                        // possible if pre-colored first vertex?
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
                return false; // dead end - konflikt

                // println!("two uncolored edges of vertex {}", vertex);
            }
            _ => {
                panic!("More than 2 uncolored edges");
            }
        }
    }

    // fn get_edge_color(&self, from: usize, to: usize) -> usize {
    //     for neighbor in self.vertices[from].iter() {
    //         if neighbor.0 == to {
    //             return neighbor.1;
    //         }
    //     }
    //     panic!("there is no edge from {} to {}", from, to);
    // }

    fn set_edge_color(&mut self, from: usize, to: usize, color: usize) {
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
