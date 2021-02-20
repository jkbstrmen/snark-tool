use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;
use crate::service::colour::colouriser::Colouriser;
use splr::Certificate;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::iter::FromIterator;

///
/// for snarks with 58 vertices is about twice as time consuming as SAT-col using Varisat
///
pub struct SATSplrColourizer {}

impl Colouriser for SATSplrColourizer {
    fn is_colorable<G>(graph: &G) -> bool
    where
        G: Graph,
    {
        let cnf_sat = Self::graph_to_cnf_sat(graph);

        // // TODO - handle panic - add Result to Colouriser trait?
        // let result = match Certificate::try_from(cnf_sat).expect("panic!") {
        //     Certificate::UNSAT => false,
        //     Certificate::SAT(vec) => true,
        // };
        // result
        false
    }

    fn new() -> Self {
        SATSplrColourizer {}
    }
}

impl SATSplrColourizer {
    fn graph_to_cnf_sat<G: Graph>(graph: &G) -> Vec<Vec<i32>> {
        let v: Vec<Vec<i32>> = vec![vec![1, 2], vec![-1, 3], vec![1, -3], vec![-1, 2]];
        let mut formula: Vec<Vec<i32>> = vec![];

        let mut edge_lits = EdgeLits::new();

        for edge in graph.edges() {
            // xij1 - if true - given edge is coloured by color 1
            let (xij1, xij2, xij3) = edge_lits.next_lits(edge.from(), edge.to());

            // every edge has to be colored exactly by one colour
            formula.push(vec![xij1, xij2, xij3]);
            formula.push(vec![-xij1, -xij2, xij3]);
            formula.push(vec![-xij1, xij2, -xij3]);
            formula.push(vec![xij1, -xij2, -xij3]);
            formula.push(vec![-xij1, -xij2, -xij3]);
        }

        // improvement - not necessary - and not helping
        // Self::add_improvement(graph, &mut formula, &edge_lits);

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
            formula.push(vec![-lits_first.0, -lits_second.0]);
            formula.push(vec![-lits_first.1, -lits_second.1]);
            formula.push(vec![-lits_first.2, -lits_second.2]);

            if edges.len() == 3 {
                let lits_third = edge_lits
                    .lits_of_edge(edges[2].from(), edges[2].to())
                    .unwrap();

                // 0, 2
                formula.push(vec![-lits_first.0, -lits_third.0]);
                formula.push(vec![-lits_first.1, -lits_third.1]);
                formula.push(vec![-lits_first.2, -lits_third.2]);

                // 1, 2
                formula.push(vec![-lits_second.0, -lits_third.0]);
                formula.push(vec![-lits_second.1, -lits_third.1]);
                formula.push(vec![-lits_second.2, -lits_third.2]);
            }
        }
        formula
    }

    fn add_improvement<G: Graph>(graph: &G, formula: &mut Vec<Vec<i32>>, edge_lits: &EdgeLits) {
        for edge in graph.edges() {
            let edges_from = graph.edges_of_vertex(edge.from());
            let edges_to = graph.edges_of_vertex(edge.to());

            // edges of from vertex
            let mut from_vertex_edges_lits = vec![];
            for edge in edges_from {
                let lits = edge_lits.lits_of_edge(edge.from(), edge.to()).unwrap();
                from_vertex_edges_lits.push(lits);
            }
            // edges of to vertex
            let mut to_vertex_edges_lits = vec![];
            for edge in edges_to {
                let lits = edge_lits.lits_of_edge(edge.from(), edge.to()).unwrap();
                to_vertex_edges_lits.push(lits);
            }

            // neighbouring edges cannot have same colour
            let mut clause_col_0 = vec![];
            let mut clause_col_1 = vec![];
            let mut clause_col_2 = vec![];
            for lits in from_vertex_edges_lits.iter() {
                clause_col_0.push(-lits.0);
                clause_col_1.push(-lits.1);
                clause_col_2.push(-lits.2);
            }
            for lits in to_vertex_edges_lits.iter() {
                clause_col_0.push(-lits.0);
                clause_col_1.push(-lits.1);
                clause_col_2.push(-lits.2);
            }
            formula.push(clause_col_0);
            formula.push(clause_col_1);
            formula.push(clause_col_2);

            // if at least one of neighbouring vertices has 3 edges, each color has to be present in neighbouring edges
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
                formula.push(clause_col_0);
                formula.push(clause_col_1);
                formula.push(clause_col_2);
            }
        }
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
