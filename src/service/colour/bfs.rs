use crate::graph::graph::Graph;

// for graphs with at most 170 vertices and 255 edges

pub struct BFSColourGraph
/*<G>*/
// where
//     G: Graph,
{
    graph: Vec<u8>,
    // neighbouring vertices foreach vertex
    neighbors: Vec<[u8; 3]>,
    // vertices with on uncolored edge
    one_edge_vert: Vec<u8>,
    // number of non colored edges of vertex - foreach vertex
    non_colored_edges: Vec<u8>,

    // number of uncolored edges in graph
    non_colored_edges_of_graph: u8,
    // shouldnt be needed ... one_edge_vert.len()
    // number of vertices with one uncolored edge
    // one_edge_vertices: u8,
    // size of graph - number of vertices
    graph_size: u8,
}

impl BFSColourGraph {
    // assumes non colored graph
    pub fn is_colorable(graph: Vec<u8>, graph_size: u8) -> bool {
        //let neighbors = Vec::with_capacity(graph_size as usize);
        let neighbors: Vec<[u8; 3]> = vec![[0; 3]; graph_size as usize];

        let non_colored_edges = vec![0; graph_size as usize];

        let mut color_graph = BFSColourGraph {
            graph,
            neighbors,
            one_edge_vert: Vec::with_capacity(graph_size as usize),
            non_colored_edges,
            non_colored_edges_of_graph: 0,
            // one_edge_vertices: 0,
            graph_size,
        };

        // pre-color first vertex ...
        let colors = [3, 4, 5];
        let mut first_vertex: u8 = 0;
        let mut from = 0;
        // could be infinite loop if graph has no edges
        loop {
            let mut k = 0;
            for to in 0..graph_size {
                if color_graph.get_edge_color(from, to) == 1 {
                    color_graph.set_edge_color(from, to, colors[k]);
                    k += 1;
                    first_vertex = to;
                }
            }
            from += 1;
            if k != 0 {
                break;
            }
        }

        // init color_graph
        let mut non_colored_edges_count: usize = 0;
        for row in 0..graph_size {
            let mut neighbor = 0;

            for column in 0..graph_size {
                let edge_col = color_graph.get_edge_color(row, column);
                if edge_col > 0 {
                    color_graph.neighbors[row as usize][neighbor] = column;

                    // let neighbors = color_graph.neighbors.get_mut(row as usize).unwrap();
                    // neighbors[neighbor] = column;
                    neighbor += 1;
                    if edge_col == 1 {
                        //color_graph.non_colored_edges_of_graph += 1;
                        //*color_graph.non_colored_edges.get_mut(row as usize).unwrap() += 1;
                        non_colored_edges_count += 1;
                        color_graph.non_colored_edges[row as usize] += 1;
                    }
                }
            }

            // why ??
            if neighbor < 3 {
                color_graph.neighbors[row as usize][neighbor] = row;
            }
            neighbor += 1;
            if neighbor < 3 {
                let neighbors = color_graph.neighbors.get_mut(row as usize).unwrap();
                neighbors[neighbor] = row;
            }
        }

        non_colored_edges_count = non_colored_edges_count / 2;
        color_graph.non_colored_edges_of_graph = non_colored_edges_count as u8;
        // count vertices with one uncolored edge
        for vertex in 0..graph_size {
            if (1 as u8).eq(color_graph.non_colored_edges.get(vertex as usize).unwrap()) {
                color_graph.one_edge_vert.push(vertex);
                // color_graph.one_edge_vertices += 1;
            }
        }

        println!();
        // color_graph.print();

        color_graph.color(first_vertex)
        // false
    }

    fn color(&mut self, vertex: u8) -> bool {
        let color_vars = [(4, 5), (3, 5), (3, 4)];

        let mut neighbor1 = 0;
        let mut neighbor2 = 0;
        let mut colored_sum: usize = 0;
        for neighbor in &self.neighbors[vertex as usize] {
            let color = self.get_edge_color(vertex, neighbor.clone());
            if color == 1 {
                neighbor2 = neighbor1.clone();
                neighbor1 = neighbor.clone();
            } else {
                colored_sum += color as usize;
            }
        }

        match self.non_colored_edges[vertex as usize] {
            0 => {
                // println!("zero uncolored edges of vertex {}", vertex);

                if self.non_colored_edges_of_graph == 0 {
                    return true;
                } else {
                    let mut vert = 0;
                    while (self.non_colored_edges[vert] != 1) && (self.non_colored_edges[vert] != 2)
                    {
                        // why?
                        vert += 1;
                    }
                    return self.color(vert as u8);
                }
            }
            1 => {
                self.non_colored_edges_of_graph -= 1;
                self.non_colored_edges[vertex as usize] -= 1;
                self.non_colored_edges[neighbor1 as usize] -= 1;
                let mut change = false;
                let mut next_vertex = 0;
                if self.non_colored_edges[neighbor1 as usize] == 1 && !self.one_edge_vert.is_empty()
                {
                    change = true;
                    next_vertex = self.one_edge_vert.pop().unwrap(); // preco sa ale sem potom do jenodhrannych neprida kde1, ked aj kde1 uz bude jednohran?
                } else {
                    next_vertex = neighbor1 as u8;
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
                            (12 - colored_sum) as u8,
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
                let mut next_vertex = 0;

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
                    0 => { // possible if pre-colored first vertex?
                        for num in 3..6 {
                            self.set_edge_color(vertex, neighbor1, color_vars[num-3].0);
                            self.set_edge_color(vertex, neighbor2, color_vars[num-3].1);
                            if self.are_vertices_without_conflict(neighbor1, neighbor2)
                                && self.color(next_vertex) { return true; }

                            self.set_edge_color(vertex, neighbor1, color_vars[num-3].1);
                            self.set_edge_color(vertex, neighbor2, color_vars[num-3].0);
                            if self.are_vertices_without_conflict(neighbor1, neighbor2)
                                && self.color(next_vertex) { return true; }
                        }
                    }
                    _ => {

                        self.set_edge_color(vertex, neighbor1, color_vars[colored_sum-3].0);
                        self.set_edge_color(vertex, neighbor2, color_vars[colored_sum-3].1);
                        if self.are_vertices_without_conflict(neighbor1, neighbor2)
                            && self.color(next_vertex) { return true; }

                        self.set_edge_color(vertex, neighbor1, color_vars[colored_sum-3].1);
                        self.set_edge_color(vertex, neighbor2, color_vars[colored_sum-3].0);
                        if self.are_vertices_without_conflict(neighbor1, neighbor2)
                            && self.color(next_vertex) { return true; }

                    }
                }

                // revert changes
                if one_edge_neighbors == 2 { self.one_edge_vert.pop(); }
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
        false
    }

    fn get_edge_color(&self, from: u8, to: u8) -> u8 {
        let index = from as usize * self.graph_size as usize + to as usize;
        self.graph.get(index as usize).unwrap().clone()
    }

    fn set_edge_color(&mut self, from: u8, to: u8, color: u8) {
        // let index = from * self.graph_size + to;
        // std::mem::replace(&mut self.graph[index as usize], color);
        self.graph[from as usize * self.graph_size as usize + to as usize] = color;
        self.graph[to as usize * self.graph_size as usize + from as usize] = color;
    }

    // ttt2
    fn is_vertex_without_conflict(&self, vertex: u8, neighbors: &[u8; 3]) -> bool {
        is_without_conflict(
            self.get_edge_color(vertex, neighbors[0]),
            self.get_edge_color(vertex, neighbors[1]),
            self.get_edge_color(vertex, neighbors[2]),
        )
    }

    // cond1
    fn set_color_and_check_validity(&mut self, from: u8, to: u8, color: u8) -> bool {
        self.set_edge_color(from, to, color);
        self.is_vertex_without_conflict(from, &self.neighbors[from as usize])
    }

    // cond2
    fn are_vertices_without_conflict(&self, first: u8, second: u8) -> bool {
        self.is_vertex_without_conflict(first, &self.neighbors[first as usize])
            && self.is_vertex_without_conflict(second, &self.neighbors[second as usize])
    }

    fn set_edge_check_and_color_rest(
        &mut self,
        from: u8,
        to: u8,
        color: u8,
        next_vertex: u8,
    ) -> bool {
        self.set_color_and_check_validity(from, to, color) && self.color(next_vertex)
    }

    // temp
    pub fn print(&self) {
        for row in 0..self.graph_size {
            for column in 0..self.graph_size {
                let index = row * self.graph_size + column;
                print!("{} ", self.graph.get(index as usize).unwrap());
            }
            println!();
        }
    }
}

fn is_without_conflict(color1: u8, color2: u8, color3: u8) -> bool {
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
