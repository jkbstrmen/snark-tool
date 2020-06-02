use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;
use crate::service::colour::colouriser::Colourizer;
use std::collections::HashMap;
use std::iter::FromIterator;
use varisat::solver::Solver;
use varisat::{CnfFormula, ExtendFormula};

pub struct SATColourizer {}

// TODO - adjust for subcubic graphs

// impl SATColourizer {
//     pub fn new() -> Self {
//         SATColourizer{}
//     }
// }

impl Colourizer for SATColourizer {
    fn is_colorable<G, V, E>(graph: &G) -> bool
    where
        G: Graph<V, E>,
        V: Vertex,
        E: Edge,
    {
        let mut solver = Solver::new();

        let mut edge_lits = HashMap::new();

        for edge in graph.edges() {
            // xij1 - if true - given edge is coloured by color 1
            let (xij1, xij2, xij3) = solver.new_lits();
            edge_lits.insert((edge.from(), edge.to()), (xij1, xij2, xij3));

            // every edge has to be colored exactly by one colour
            let mut formula = CnfFormula::new();
            formula.add_clause(&[xij1, xij2, xij3]);
            formula.add_clause(&[!xij1, !xij2, xij3]);
            formula.add_clause(&[!xij1, xij2, !xij3]);
            formula.add_clause(&[xij1, !xij2, !xij3]);
            formula.add_clause(&[!xij1, !xij2, !xij3]);

            solver.add_formula(&formula);
        }

        // improvement - not necessary
        for edge in graph.edges() {
            // TODO - adjust for subcubic graphs

            let edges_from = graph.edges_of_vertex(edge.from());
            // let edges_from: Vec<&E> = Vec::from_iter(edges_from);
            let edges_to = graph.edges_of_vertex(edge.to());
            // let edges_to: Vec<&E> = Vec::from_iter(edges_to);

            // // edges of from vertex
            // let first_edge_lits = edge_lits
            //     .get(&(edges_from[0].from(), edges_from[0].to()))
            //     .unwrap();
            // let second_edge_lits = edge_lits
            //     .get(&(edges_from[1].from(), edges_from[1].to()))
            //     .unwrap();
            // let third_edge_lits = edge_lits
            //     .get(&(edges_from[2].from(), edges_from[2].to()))
            //     .unwrap();
            //
            // // edges of to vertex
            // let fourth_edge_lits = edge_lits
            //     .get(&(edges_to[0].from(), edges_to[0].to()))
            //     .unwrap();
            // let fifth_edge_lits = edge_lits
            //     .get(&(edges_to[1].from(), edges_to[1].to()))
            //     .unwrap();
            // let sixth_edge_lits = edge_lits
            //     .get(&(edges_to[2].from(), edges_to[2].to()))
            //     .unwrap();

            // edges of from vertex
            let mut from_vertex_edges_lits = vec![];
            for edge in edges_from {
                let lits = edge_lits
                    .get(&(edge.from(), edge.to()))
                    .unwrap();
                from_vertex_edges_lits.push(lits);
            }
            // edges of to vertex
            let mut to_vertex_edges_lits = vec![];
            for edge in edges_to {
                let lits = edge_lits
                    .get(&(edge.from(), edge.to()))
                    .unwrap();
                to_vertex_edges_lits.push(lits);
            }

            // TODO - adjust for subcubic graphs

            let mut formula = CnfFormula::new();

            // neighbouring edges cannot have same colour
            let mut clause_col_0 = vec![];
            let mut clause_col_1 = vec![];
            let mut clause_col_2 = vec![];
            for lits in from_vertex_edges_lits.iter() {
                clause_col_0.push(!lits.0);
                clause_col_1.push(!lits.1);
                clause_col_2.push(!lits.2);
            }
            for lits in to_vertex_edges_lits.iter() {
                clause_col_0.push(!lits.0);
                clause_col_1.push(!lits.1);
                clause_col_2.push(!lits.2);
            }
            formula.add_clause(&clause_col_0);
            formula.add_clause(&clause_col_1);
            formula.add_clause(&clause_col_2);

            // if at least one of neighbouring vertices has 3 edges, each color has to present in neighbouring edges
            if from_vertex_edges_lits.len() == 3 || to_vertex_edges_lits.len() == 3 {
                let mut clause_col_0 = vec![];
                let mut clause_col_1 = vec![];
                let mut clause_col_2 = vec![];
                for lits in from_vertex_edges_lits.iter() {
                    clause_col_0.push(lits.0);
                    clause_col_1.push(lits.1);
                    clause_col_2.push(lits.2);
                }
                for lits in to_vertex_edges_lits.iter() {
                    clause_col_0.push(lits.0);
                    clause_col_1.push(lits.1);
                    clause_col_2.push(lits.2);
                }
                formula.add_clause(&clause_col_0);
                formula.add_clause(&clause_col_1);
                formula.add_clause(&clause_col_2);
            }


            // // cannot be all edges colour with the same colour 0
            // formula.add_clause(&[
            //     !first_edge_lits.0,
            //     !second_edge_lits.0,
            //     !third_edge_lits.0,
            //     !fourth_edge_lits.0,
            //     !fifth_edge_lits.0,
            //     !sixth_edge_lits.0,
            // ]);
            // // at least one of edges has to be coloured by color 0
            // formula.add_clause(&[
            //     first_edge_lits.0,
            //     second_edge_lits.0,
            //     third_edge_lits.0,
            //     fourth_edge_lits.0,
            //     fifth_edge_lits.0,
            //     sixth_edge_lits.0,
            // ]);
            // formula.add_clause(&[
            //     !first_edge_lits.1,
            //     !second_edge_lits.1,
            //     !third_edge_lits.1,
            //     !fourth_edge_lits.1,
            //     !fifth_edge_lits.1,
            //     !sixth_edge_lits.1,
            // ]);
            // formula.add_clause(&[
            //     first_edge_lits.1,
            //     second_edge_lits.1,
            //     third_edge_lits.1,
            //     fourth_edge_lits.1,
            //     fifth_edge_lits.1,
            //     sixth_edge_lits.1,
            // ]);
            // formula.add_clause(&[
            //     !first_edge_lits.2,
            //     !second_edge_lits.2,
            //     !third_edge_lits.2,
            //     !fourth_edge_lits.2,
            //     !fifth_edge_lits.2,
            //     !sixth_edge_lits.2,
            // ]);
            // formula.add_clause(&[
            //     first_edge_lits.2,
            //     second_edge_lits.2,
            //     third_edge_lits.2,
            //     fourth_edge_lits.2,
            //     fifth_edge_lits.2,
            //     sixth_edge_lits.2,
            // ]);

            solver.add_formula(&formula);
        }

        for vertex in graph.vertices() {
            let edges = graph.edges_of_vertex(vertex.index());
            let edges: Vec<&E> = Vec::from_iter(edges);

            if edges.is_empty() || edges.len() == 1 {
                continue;
            }

            let lits_first = edge_lits.get(&(edges[0].from(), edges[0].to())).unwrap();
            let lits_second = edge_lits.get(&(edges[1].from(), edges[1].to())).unwrap();

            let mut formula = CnfFormula::new();

            // 0, 1 - edges 0 and 1 of vertex cannot have same colour
            formula.add_clause(&[!lits_first.0, !lits_second.0]);
            formula.add_clause(&[!lits_first.1, !lits_second.1]);
            formula.add_clause(&[!lits_first.2, !lits_second.2]);

            if edges.len() == 3 {
                let lits_third = edge_lits.get(&(edges[2].from(), edges[2].to())).unwrap();

                // 0, 2
                formula.add_clause(&[!lits_first.0, !lits_third.0]);
                formula.add_clause(&[!lits_first.1, !lits_third.1]);
                formula.add_clause(&[!lits_first.2, !lits_third.2]);

                // 1, 2
                formula.add_clause(&[!lits_second.0, !lits_third.0]);
                formula.add_clause(&[!lits_second.1, !lits_third.1]);
                formula.add_clause(&[!lits_second.2, !lits_third.2]);
            }
            solver.add_formula(&formula);
        }

        solver.solve().unwrap()
    }

    fn new() -> Self {
        SATColourizer{}
    }

    // todo - temp
    fn is_colorable_with_counter<G, V, E>(graph: &G, counter: &mut usize) -> bool where
        G: Graph<V, E>,
        V: Vertex,
        E: Edge {
        unimplemented!()
    }
}
