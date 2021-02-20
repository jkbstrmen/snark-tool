use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;
use crate::service::colour::colouriser::Colouriser;
use crate::service::colour::sat_new_2::ELAPSED;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::time::Instant;
use varisat::solver::Solver;
use varisat::{CnfFormula, ExtendFormula, Lit};

pub struct SATColourizerCadical {}

impl Colouriser for SATColourizerCadical {
    fn is_colorable<G>(graph: &G) -> bool
    where
        G: Graph,
    {
        let mut solver = Self::graph_to_cnf_sat(graph);
        solver.solve().unwrap()
    }

    fn new() -> Self {
        SATColourizerCadical {}
    }
}

impl SATColourizerCadical {
    fn graph_to_cnf_sat<G: Graph>(graph: &G) -> cadical::Solver {
        let mut solver: cadical::Solver = Default::default();

        let mut edge_lits = EdgeLits::new();

        for edge in graph.edges() {
            // xij1 - if true - given edge is coloured by color 1
            let (xij1, xij2, xij3) = edge_lits.next_lits(edge.from(), edge.to());

            // every edge has to be colored exactly by one colour
            solver.add_clause([xij1, xij2, xij3].iter().copied());
            solver.add_clause([-xij1, -xij2, xij3].iter().copied());
            solver.add_clause([-xij1, xij2, -xij3].iter().copied());
            solver.add_clause([xij1, -xij2, -xij3].iter().copied());
            solver.add_clause([-xij1, -xij2, -xij3].iter().copied());
        }

        for vertex in graph.vertices() {
            let edges = graph.edges_of_vertex(vertex.index());
            let edges: Vec<&G::E> = Vec::from_iter(edges);

            if edges.is_empty() || edges.len() == 1 {
                continue;
            }

            let lits_first = edge_lits
                .lits_of_edge(edges[0].from(), edges[0].to())
                .unwrap();
            let lits_second = edge_lits
                .lits_of_edge(edges[1].from(), edges[1].to())
                .unwrap();

            // 0, 1 - edges 0 and 1 of vertex cannot have same colour
            solver.add_clause([-lits_first.0, -lits_second.0].iter().copied());
            solver.add_clause([-lits_first.1, -lits_second.1].iter().copied());
            solver.add_clause([-lits_first.2, -lits_second.2].iter().copied());

            if edges.len() == 3 {
                let lits_third = edge_lits
                    .lits_of_edge(edges[2].from(), edges[2].to())
                    .unwrap();

                // 0, 2
                solver.add_clause([-lits_first.0, -lits_third.0].iter().copied());
                solver.add_clause([-lits_first.1, -lits_third.1].iter().copied());
                solver.add_clause([-lits_first.2, -lits_third.2].iter().copied());

                // 1, 2
                solver.add_clause([-lits_second.0, -lits_third.0].iter().copied());
                solver.add_clause([-lits_second.1, -lits_third.1].iter().copied());
                solver.add_clause([-lits_second.2, -lits_third.2].iter().copied());
            }
        }
        solver
    }
}

struct EdgeLits {
    // edge -> literals of edge (one for each possible colour)
    edge_lits: HashMap<(usize, usize), (i32, i32, i32)>,
    lits: Vec<usize>,
}

impl EdgeLits {
    pub fn new() -> Self {
        let mut lits = EdgeLits {
            edge_lits: HashMap::new(),
            lits: Vec::new(),
        };
        lits.lits.push(0);
        lits
    }

    pub fn next_lits(&mut self, from: usize, to: usize) -> (i32, i32, i32) {
        let mut lits = (0, 0, 0);

        lits.0 = self.lits.len() as i32;
        self.lits.push(self.lits.len());
        lits.1 = self.lits.len() as i32;
        self.lits.push(self.lits.len());
        lits.2 = self.lits.len() as i32;
        self.lits.push(self.lits.len());
        self.edge_lits.insert((from, to), lits);

        lits
    }

    pub fn lits_of_edge(&self, from: usize, to: usize) -> Option<&(i32, i32, i32)> {
        self.edge_lits.get(&(from, to))
    }
}
