use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;

use rand::seq::SliceRandom;
use std::collections::VecDeque;
use std::cmp;

pub static NON_COLOURED_EDGE: u8 = 0;

// #[derive(Clone)]
struct CVDGraph {
    // Vec of vertices, where vertex is Vec of neighbors, where neighbor is (usize, u8) as (index, colour)
    // TODO - try with fixed size array [(usize, u8), 3]
    vertices: Vec<Vec<(usize, u8)>>,
    vertices_to_try: Vec<usize>,
    kempe_chain: KempeChain,
    conflicting_vertices: Vec<Vec<usize>>,
}

// #[derive(Clone)]
struct KempeChain {
    vertices: Vec<usize>,
    conflicting_colour: u8,
    resolving_colour: u8,
}

struct KempeStepResult {
    next_vertex: usize,
    old_colour: u8,
    chain_enclosed: bool,
}

const L_LIMIT: usize = 50;
const R_LIMIT: usize = 50;

///
///
///
pub fn is_colorable<G>(graph: &G) -> Option<bool>
where
    G: Graph,
{
    let mut graph = create_cvd_graph(graph);

    let mut l_limit = graph.vertices_to_try.len() / 2;
    l_limit = cmp::min(l_limit, L_LIMIT);
    let mut r_limit = graph.vertices_to_try.len() / 4;
    r_limit = cmp::min(r_limit, R_LIMIT);

    for _i in 0..l_limit {
        graph.next_pre_colour();

        let colorable = graph.is_colorable(r_limit);

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
        for edge in graph.edges_of_vertex(vertex.index()) {
            if edge.from() == vertex.index() {
                // vertex_new[i] = (edge.to(), 1);
                vertex_new.push((edge.to(), NON_COLOURED_EDGE));
            } else {
                // vertex_new[i] = (edge.from(), 1);
                vertex_new.push((edge.from(), NON_COLOURED_EDGE));
            }
        }
        vertices.push(vertex_new);
    }
    let mut vertices_to_try = Vec::with_capacity(graph.size());
    for i in 0..graph.size() {
        if graph.edges_of_vertex(i).next().is_some() {
            vertices_to_try.push(i);
        }
    }

    CVDGraph {
        vertices,
        vertices_to_try,
        kempe_chain: KempeChain {
            vertices: vec![],
            conflicting_colour: 0,
            resolving_colour: 0,
        },
        conflicting_vertices: vec![vec![]; 3],
    }
}

impl CVDGraph {
    fn is_colorable(&mut self, r_limit: usize) -> bool {
        let mut repetition_counter = 0;
        self.conflicting_vertices = self.conflicting_vertices();
        let mut total_number_of_conflicts = self.total_number_of_conflicts();

        while total_number_of_conflicts > 0 {
            let previous = total_number_of_conflicts;

            // faster for smaller graphs
            // let next_vertex = self.highest_conflicting_vertex();
            let next_vertex = self.random_highest_conflicting_vertex();

            self.kempe_chain_swap(next_vertex);

            total_number_of_conflicts = self.total_number_of_conflicts();
            if total_number_of_conflicts >= previous {
                repetition_counter += 1;
            } else {
                repetition_counter = 0;
            }
            if repetition_counter > r_limit {
                return false;
            }
        }
        true
    }

    fn random_highest_conflicting_vertex(&self) -> usize {
        if self.conflicting_vertices[2].is_empty() {
            return *self.conflicting_vertices[1]
                .choose(&mut rand::thread_rng())
                .unwrap();
        }
        *self.conflicting_vertices[2]
            .choose(&mut rand::thread_rng())
            .unwrap()
    }

    ///
    /// faster for smaller graphs
    ///
    #[allow(dead_code)]
    fn highest_conflicting_vertex(&self) -> usize {
        if self.conflicting_vertices[2].is_empty() {
            return self.conflicting_vertices[1][0];
        }
        self.conflicting_vertices[2][0]
    }

    fn next_pre_colour(&mut self) {
        // clean graph colors
        for vertex in self.vertices.iter_mut() {
            for neighbor in vertex.iter_mut() {
                neighbor.1 = NON_COLOURED_EDGE;
            }
        }
        let first_vertex = self.next_random_vertex();
        // let first_vertex = 0;
        unsafe {
            self.bfs_pre_colour(first_vertex);
        }
    }

    unsafe fn bfs_pre_colour(&mut self, first_vertex: usize) {
        let self_ref = self as *const CVDGraph;
        let mut bfs_graph = BfsGraph::new_from_raw_ptr(self_ref, first_vertex);

        while let Some(vertex) = bfs_graph.bfs_next() {
            let mut available = self.available_colors_of_vertex(vertex);
            for neighbor in (*self_ref).vertices[vertex].iter() {
                if neighbor.1 == NON_COLOURED_EDGE {
                    let color = available.pop().unwrap();
                    self.set_edge_color(vertex, neighbor.0, color);
                }
            }
        }
    }

    fn colour_edge_and_update_conflicting_vertices(
        &mut self,
        from: usize,
        to: usize,
        color: u8,
    ) -> bool {
        let old_conflict_level_from = self.conflict_level(&self.vertices[from]);
        let old_conflict_level_to = self.conflict_level(&self.vertices[to]);
        self.set_edge_color(from, to, color);
        self.update_conflict_vertices(from, old_conflict_level_from);
        self.update_conflict_vertices(to, old_conflict_level_to)
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

    fn edge_color(&self, from: usize, to: usize) -> u8 {
        for neighbor in self.vertices[from].iter() {
            if neighbor.0 == to {
                return neighbor.1;
            }
        }
        0
    }

    ///
    /// if returns true, conflict level of vertex decreased
    ///
    fn update_conflict_vertices(&mut self, vertex: usize, old_conflict_level: usize) -> bool {
        let conflict_level = self.conflict_level(&self.vertices[vertex]);
        // remove vertex from old conflict level
        if old_conflict_level > 0 {
            self.conflicting_vertices[old_conflict_level].retain(|item| item != &vertex);
        }
        // add vertex to new conflict level
        if conflict_level > 0 {
            self.conflicting_vertices[conflict_level].push(vertex);
        }
        // to later use
        if (conflict_level as isize - old_conflict_level as isize) < 0 {
            return true;
        }
        false
    }

    fn available_colors_of_vertex(&self, vertex: usize) -> Vec<u8> {
        let mut colors = vec![3, 4, 5];
        for neighbor in self.vertices[vertex].iter() {
            colors.retain(|&item| item != neighbor.1)
        }
        colors
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

    fn conflicting_vertices(&self) -> Vec<Vec<usize>> {
        let mut cvs = vec![vec![]; 3];
        let mut index = 0;
        for vertex in self.vertices.iter() {
            let conflict_level = self.conflict_level(vertex);
            if conflict_level > 0 {
                cvs[conflict_level].push(index);
            }
            index += 1;
        }
        cvs
    }

    fn total_number_of_conflicts(&self) -> usize {
        return self.conflicting_vertices[1].len() + self.conflicting_vertices[2].len();
    }

    fn kempe_chain_swap(&mut self, start_vertex: usize) {
        self.kempe_chain.vertices = vec![];
        self.kempe_chain.resolving_colour = 0;
        self.kempe_chain.conflicting_colour = 0;

        let conflicting_colour = self.conflicting_color_of_vertex(start_vertex).unwrap();
        let mut resolving_colour = self.available_colors_of_vertex(start_vertex)[0];
        self.kempe_chain.conflicting_colour = conflicting_colour;
        self.kempe_chain.resolving_colour = resolving_colour;

        let mut next = None;
        for neighbors in self.vertices[start_vertex].iter() {
            if neighbors.1 == self.kempe_chain.conflicting_colour {
                next = Some(neighbors.0);
                break;
            }
        }

        if next != None {
            let mut current_vertex = start_vertex;
            let mut next_vertex = next.unwrap();

            loop {
                self.kempe_chain.vertices.push(current_vertex);
                let step_result = self.kempe_step(current_vertex, next_vertex, resolving_colour);
                if step_result.chain_enclosed {
                    return;
                }
                current_vertex = next_vertex;
                next_vertex = step_result.next_vertex;
                resolving_colour = step_result.old_colour;

                // check if current_vertex not already in chain
                if let Some(_vertex) = self
                    .kempe_chain
                    .vertices
                    .iter()
                    .find(|vertex| vertex == &&current_vertex)
                {
                    return;
                }
            }
        }
    }

    ///
    /// new_colour is colour for edge between current_vertex and last vertex of chain
    /// if returns true - we enclosed kempe chain
    ///
    fn kempe_step(
        &mut self,
        previous_vertex: usize,
        current_vertex: usize,
        new_colour: u8,
    ) -> KempeStepResult {
        let mut result = KempeStepResult {
            next_vertex: 0,
            old_colour: 0,
            chain_enclosed: true,
        };

        let available_for_next =
            self.available_for_next_vertex(previous_vertex, current_vertex, new_colour);

        result.old_colour = self.edge_color(previous_vertex, current_vertex);
        let decreased_conflict_level = self.colour_edge_and_update_conflicting_vertices(
            previous_vertex,
            current_vertex,
            new_colour,
        );
        if decreased_conflict_level {
            return result;
        }
        if available_for_next.is_empty() {
            return result;
        }
        result.next_vertex = available_for_next[0];
        result.chain_enclosed = false;
        result
    }

    fn available_for_next_vertex(
        &self,
        previous: usize,
        current: usize,
        new_colour: u8,
    ) -> Vec<usize> {
        let mut available = vec![];
        for vertex in self.vertices[current].iter() {
            if vertex.0 != previous && vertex.1 == new_colour {
                available.push(vertex.0);
            }
        }
        available
    }

    fn conflict_level(&self, vertex: &Vec<(usize, u8)>) -> usize {
        match vertex.len() {
            0 => {
                return 0;
            }
            1 => {
                return 0;
            }
            2 => {
                if vertex[0].1 == vertex[1].1 {
                    return 1;
                }
                return 0;
            }
            3 => {
                let mut conflict_level = 0;
                if vertex[0].1 == vertex[1].1 {
                    conflict_level += 1;
                }
                if vertex[0].1 == vertex[2].1 {
                    conflict_level += 1;
                }
                if vertex[1].1 == vertex[2].1 {
                    conflict_level += 1;
                }
                if conflict_level > 2 {
                    return 2;
                }
                return conflict_level;
            }
            _ => panic!("vertex has more than 3 neighbors"),
        }
    }
}

// TODO - deduplicate with BfsGraph from perfect_matchings

struct BfsGraph<'a> {
    vertices: &'a Vec<Vec<(usize, u8)>>,
    visited: Vec<bool>,
    to_visit: VecDeque<usize>,
}

impl<'a> BfsGraph<'a> {
    // pub fn new(graph: &'a CVDGraph, start: usize) -> Self {
    //     let visited = vec![false; graph.vertices.len()];
    //     let mut to_visit = VecDeque::new();
    //     to_visit.push_back(start);
    //
    //     let mut bfs = Self {
    //         vertices: &graph.vertices,
    //         visited,
    //         to_visit,
    //     };
    //     bfs.visit(start);
    //     bfs
    // }

    pub unsafe fn new_from_raw_ptr(graph: *const CVDGraph, start: usize) -> Self {
        let visited = vec![false; (*graph).vertices.len()];
        let mut to_visit = VecDeque::new();
        to_visit.push_back(start);

        let mut bfs = Self {
            vertices: &(*graph).vertices,
            visited,
            to_visit,
        };
        bfs.visit(start);
        bfs
    }

    ///
    /// if true, visited for the first time
    ///
    fn visit(&mut self, vertex: usize) -> bool {
        let old_val = self.visited[vertex];
        self.visited[vertex] = true;
        !old_val
    }

    pub fn bfs_next(&mut self) -> Option<usize> {
        if let Some(vertex) = self.to_visit.pop_front() {
            for neighbor in self.vertices[vertex].iter() {
                if self.visit(neighbor.0) {
                    self.to_visit.push_back(neighbor.0);
                }
            }
            return Some(vertex);
        }
        None
    }
}
