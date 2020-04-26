use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;
use std::collections::HashMap;
use std::iter::FromIterator;
use varisat::solver::Solver;
use varisat::{CnfFormula, ExtendFormula};

// TODO - try alternative clauses for neighboring edges

pub fn is_colorable<G, V, E>(graph: &G) -> bool
where
    G: Graph<V, E>,
    V: Vertex,
    E: Edge + Eq, /*+ Hash + Debug*/
{
    let mut solver = Solver::new();

    let mut edge_lits = HashMap::new();

    for edge in graph.edges() {
        let (xij1, xij2, xij3) = solver.new_lits();
        edge_lits.insert((edge.from(), edge.to()), (xij1, xij2, xij3));

        let mut formula = CnfFormula::new();
        formula.add_clause(&[xij1, xij2, xij3]);
        formula.add_clause(&[!xij1, !xij2, xij3]);
        formula.add_clause(&[!xij1, xij2, !xij3]);
        formula.add_clause(&[xij1, !xij2, !xij3]);
        formula.add_clause(&[!xij1, !xij2, !xij3]);

        solver.add_formula(&formula);
    }

    for edge in graph.edges() {
        let edges_from = graph.edges_of_vertex(edge.from());
        let edges_from: Vec<&E> = Vec::from_iter(edges_from);
        let edges_to = graph.edges_of_vertex(edge.to());
        let edges_to: Vec<&E> = Vec::from_iter(edges_to);

        // edges of from vertex
        let first_edge_lits = edge_lits
            .get(&(edges_from[0].from(), edges_from[0].to()))
            .unwrap();
        let second_edge_lits = edge_lits
            .get(&(edges_from[1].from(), edges_from[1].to()))
            .unwrap();
        let third_edge_lits = edge_lits
            .get(&(edges_from[2].from(), edges_from[2].to()))
            .unwrap();

        // edges of to vertex
        let fourth_edge_lits = edge_lits
            .get(&(edges_to[0].from(), edges_to[0].to()))
            .unwrap();
        let fifth_edge_lits = edge_lits
            .get(&(edges_to[1].from(), edges_to[1].to()))
            .unwrap();
        let sixth_edge_lits = edge_lits
            .get(&(edges_to[2].from(), edges_to[2].to()))
            .unwrap();

        // adjust for subcubic graphs

        let mut formula = CnfFormula::new();
        // cannot be all edges colour with the same colour
        formula.add_clause(&[
            !first_edge_lits.0,
            !second_edge_lits.0,
            !third_edge_lits.0,
            !fourth_edge_lits.0,
            !fifth_edge_lits.0,
            !sixth_edge_lits.0,
        ]);
        // at least one of edges has to be coloured by color 0
        formula.add_clause(&[
            first_edge_lits.0,
            second_edge_lits.0,
            third_edge_lits.0,
            fourth_edge_lits.0,
            fifth_edge_lits.0,
            sixth_edge_lits.0,
        ]);
        formula.add_clause(&[
            !first_edge_lits.1,
            !second_edge_lits.1,
            !third_edge_lits.1,
            !fourth_edge_lits.1,
            !fifth_edge_lits.1,
            !sixth_edge_lits.1,
        ]);
        formula.add_clause(&[
            first_edge_lits.1,
            second_edge_lits.1,
            third_edge_lits.1,
            fourth_edge_lits.1,
            fifth_edge_lits.1,
            sixth_edge_lits.1,
        ]);
        formula.add_clause(&[
            !first_edge_lits.2,
            !second_edge_lits.2,
            !third_edge_lits.2,
            !fourth_edge_lits.2,
            !fifth_edge_lits.2,
            !sixth_edge_lits.2,
        ]);
        formula.add_clause(&[
            first_edge_lits.2,
            second_edge_lits.2,
            third_edge_lits.2,
            fourth_edge_lits.2,
            fifth_edge_lits.2,
            sixth_edge_lits.2,
        ]);

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
