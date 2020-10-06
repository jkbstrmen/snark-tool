use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;
use crate::service::colour::colouriser::Colourizer;

pub struct BFSColourizer2 {}

struct BFSColourizerGraph2 {
    graph: Vec<usize>,
    neighbors: Vec<[usize; 3]>,

    one_edge_vert: Vec<usize>,
    non_colored_edges_count: Vec<usize>,
    non_colored_edges_of_graph: usize,
    graph_size: usize,
}

impl Colourizer for BFSColourizer2 {
    fn is_colorable<G: Graph>(graph: &G) -> bool {
        // create local graph
        let mut graph_matrix = vec![0; graph.size() * graph.size()];
        let mut colour_graph = BFSColourizerGraph2 {
            graph: graph_matrix,
            neighbors: vec![[0, 0, 0]; graph.size()],
            one_edge_vert: vec![],
            non_colored_edges_count: vec![0; graph.size()],
            non_colored_edges_of_graph: 0,
            graph_size: graph.size(),
        };
        for edge in graph.edges() {
            colour_graph.set_edge_color(edge.from(), edge.to(), 1);
        }

        // precolor first vertex
        let mut first_vertex = 0;
        let colours = [3, 4, 5];
        let mut vertex_coloured = false;
        let mut vertex = 0;
        let mut colour_position = 0;
        while !vertex_coloured {
            for neighbor in 0..graph.size() {
                if colour_graph.get_edge_color(vertex, neighbor) == 1 {
                    colour_graph.set_edge_color(vertex, neighbor, colours[colour_position]);
                    colour_position += 1;
                    first_vertex = vertex;
                    vertex_coloured = true;
                }
            }
            vertex += 1;
        }
        colour_graph.non_colored_edges_of_graph = 0;

        // get count of non colored edges for whole graph and for vertices, init neighbors
        for vertex in 0..graph.size() {
            let mut neighbor_position = 0;
            colour_graph.non_colored_edges_count[vertex] = 0;
            for neighbor in 0..graph.size() {
                if colour_graph.get_edge_color(vertex, neighbor) > 0 {
                    colour_graph.neighbors[vertex][neighbor_position] = neighbor;
                    neighbor_position += 1;
                    if colour_graph.get_edge_color(vertex, neighbor) == 1 {
                        colour_graph.non_colored_edges_of_graph += 1;
                        colour_graph.non_colored_edges_count[vertex] += 1;
                    }
                }
            }
            if (neighbor_position < 3) {
                colour_graph.neighbors[vertex][neighbor_position] = vertex;
            }
            neighbor_position += 1;
            if (neighbor_position < 3) {
                colour_graph.neighbors[vertex][neighbor_position] = vertex;
            }
        }

        colour_graph.non_colored_edges_of_graph = colour_graph.non_colored_edges_of_graph / 2;
        for vertex in 0..graph.size() {
            if colour_graph.non_colored_edges_count[vertex] == 1 {
                colour_graph.one_edge_vert.push(vertex);
            }
        }
        colour_graph.color(first_vertex)
    }

    fn new() -> Self {
        Self{}
    }
}

impl BFSColourizer2 {}

impl BFSColourizerGraph2 {
    fn color(&mut self, vertex: usize) -> bool {
        let color_vars = [(4, 5), (3, 5), (3, 4)];

        let mut neighbor1 = 0;
        let mut neighbor2 = 0;
        let mut colored_sum: usize = 0;
        // find out non coloured neighbors of vertex
        for neighbor in self.neighbors[vertex].iter() {
            let color = self.get_edge_color(vertex, *neighbor);
            if color == 1 {
                neighbor2 = neighbor1;
                neighbor1 = *neighbor;
            } else {
                colored_sum += color;
            }
        }

        match self.non_colored_edges_count[vertex] {
            0 => {
                if self.non_colored_edges_of_graph == 0 {
                    return true;
                } else {
                    let mut vert = 0;
                    while (self.non_colored_edges_count[vert] != 1)
                        && (self.non_colored_edges_count[vert] != 2)
                    {
                        vert += 1;
                    }
                    return self.color(vert);
                }
            }
            1 => {
                self.non_colored_edges_of_graph -= 1;
                self.non_colored_edges_count[vertex as usize] -= 1;
                self.non_colored_edges_count[neighbor1 as usize] -= 1;
                let mut change = false;
                let next_vertex;
                if self.non_colored_edges_count[neighbor1 as usize] == 1
                    && !self.one_edge_vert.is_empty()
                {
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
                self.non_colored_edges_count[vertex as usize] += 1;
                self.non_colored_edges_count[neighbor1 as usize] += 1;
                self.set_edge_color(vertex, neighbor1, 1);
                return false;
            }
            2 => {
                self.non_colored_edges_of_graph -= 2;
                self.non_colored_edges_count[vertex as usize] -= 2;
                self.non_colored_edges_count[neighbor1 as usize] -= 1;
                self.non_colored_edges_count[neighbor2 as usize] -= 1;
                let mut one_edge_neighbors = 0;
                let mut next_from_queue = false;
                let next_vertex;

                if self.non_colored_edges_count[neighbor1 as usize] == 1 {
                    one_edge_neighbors += 1;
                    self.one_edge_vert.push(neighbor1);
                }
                if self.non_colored_edges_count[neighbor2 as usize] == 1 {
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
                self.non_colored_edges_count[vertex as usize] += 2;
                self.non_colored_edges_count[neighbor1 as usize] += 1;
                self.non_colored_edges_count[neighbor2 as usize] += 1;
                self.set_edge_color(vertex, neighbor1, 1);
                self.set_edge_color(vertex, neighbor2, 1);
                return false;
            }
            _ => {
                panic!("More than 2 uncolored edges");
            }
        }
    }

    // mmat_s2
    fn set_edge_color(&mut self, from: usize, to: usize, color: usize) {
        self.graph[from * self.graph_size + to] = color;
        self.graph[to * self.graph_size + from] = color;
    }

    // mmat_g
    fn get_edge_color(&self, from: usize, to: usize) -> usize {
        self.graph[from * self.graph_size + to]
    }

    // ttt2
    // fn is_vertex_without_conflict(&self, neighbors: &[(usize, usize); 3]) -> bool {
    fn is_vertex_without_conflict(&self, vertex: usize, neighbors: &[usize; 3]) -> bool {
        is_without_conflict(
            self.get_edge_color(vertex, neighbors[0]),
            self.get_edge_color(vertex, neighbors[1]),
            self.get_edge_color(vertex, neighbors[2]),
        )
        // is_without_conflict(neighbors[0].1, neighbors[1].1, neighbors[2].1)
    }

    // cond1
    fn set_color_and_check_validity(&mut self, from: usize, to: usize, color: usize) -> bool {
        self.set_edge_color(from, to, color);
        self.is_vertex_without_conflict(from, &self.neighbors[from])
    }

    // cond2
    fn are_vertices_without_conflict(&self, first: usize, second: usize) -> bool {
        self.is_vertex_without_conflict(first, &self.neighbors[first])
            && self.is_vertex_without_conflict(second, &self.neighbors[second])
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
