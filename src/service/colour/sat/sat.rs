use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;
use crate::service::colour::colouriser::Colouriser;
use std::collections::HashMap;
use std::iter::FromIterator;
use varisat::solver::Solver;
use varisat::{CnfFormula, ExtendFormula, Lit};

pub struct SATColourizer {}

impl Colouriser for SATColourizer {
    fn is_colorable<G>(graph: &G) -> bool
    where
        G: Graph,
    {
        let mut solver = Self::graph_to_cnf_sat(graph);
        solver.solve().unwrap()
    }

    fn new() -> Self {
        SATColourizer {}
    }
}

impl SATColourizer {
    fn graph_to_cnf_sat<G: Graph>(graph: &G) -> Solver {
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

        for vertex in graph.vertices() {
            let edges = graph.edges_of_vertex(vertex.index());
            let edges: Vec<&G::E> = Vec::from_iter(edges);

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
        solver
    }
}
