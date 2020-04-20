use varisat::solver::Solver;
use varisat::{CnfFormula, ExtendFormula};
use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;
use crate::graph::edge::Edge;
use std::collections::HashMap;
use std::hash::Hash;
use serde::export::fmt::Debug;
use std::iter::FromIterator;


// TODO - try alternative clauses for neighboring edges


pub fn is_colorable<G, V, E>(graph: &G) -> bool
    where
        G: Graph<V, E>,
        V: Vertex,
        E: Edge + Eq /*+ Hash + Debug*/,
{
    let mut solver = Solver::new();

    let mut edge_lits = HashMap::new();

    for edge in graph.edges() {
        let (xij1, xij2, xij3) = solver.new_lits();
        edge_lits.insert((edge.from(), edge.to()), ((xij1, xij2, xij3)));

        let mut formula = CnfFormula::new();
        formula.add_clause(&[xij1, xij2, xij3]);
        formula.add_clause(&[!xij1, !xij2, xij3]);
        formula.add_clause(&[!xij1, xij2, !xij3]);
        formula.add_clause(&[xij1, !xij2, !xij3]);
        formula.add_clause(&[!xij1, !xij2, !xij3]);

        solver.add_formula(&formula);
    }

    for vertex in graph.vertices() {
        let edges = graph.edges_of_vertex(vertex.index());
        let edges: Vec<&E> = Vec::from_iter(edges);

        let lits_first = edge_lits.get(&(edges[0].from(), edges[0].to())).unwrap();
        let lits_second = edge_lits.get(&(edges[1].from(), edges[1].to())).unwrap();
        let lits_third = edge_lits.get(&(edges[2].from(), edges[2].to())).unwrap();

        let mut formula = CnfFormula::new();

        // 0, 1
        formula.add_clause(&[!lits_first.0, !lits_second.0]);
        formula.add_clause(&[!lits_first.1, !lits_second.1]);
        formula.add_clause(&[!lits_first.2, !lits_second.2]);

        // 0, 2
        formula.add_clause(&[!lits_first.0, !lits_third.0]);
        formula.add_clause(&[!lits_first.1, !lits_third.1]);
        formula.add_clause(&[!lits_first.2, !lits_third.2]);

        // 1, 2
        formula.add_clause(&[!lits_second.0, !lits_third.0]);
        formula.add_clause(&[!lits_second.1, !lits_third.1]);
        formula.add_clause(&[!lits_second.2, !lits_third.2]);


        solver.add_formula(&formula);
    }

    solver.solve().unwrap()
}


// pub fn resolve() {
//
//     let mut solver = Solver::new();
//
//
//     let (x, y, z) = solver.new_lits();
//
//     let mut formula = CnfFormula::new();
//     formula.add_clause(&[x, y, z]);
//     formula.add_clause(&[!x, !y]);
//     formula.add_clause(&[!y, !z]);
//
//     solver.add_formula(&formula);
//
//     let solution = solver.solve().unwrap();
//
//     println!("solution: {}", solution);
//
//     assert_eq!(solution, true); // satisfiable
//
//
// }