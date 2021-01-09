use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;
use crate::service::colour::colouriser::Colouriser;
use std::collections::HashMap;
use std::iter::FromIterator;
use varisat::solver::Solver;
use varisat::{CnfFormula, ExtendFormula, Lit};

pub struct SATColourizerNew {}

impl Colouriser for SATColourizerNew {
    fn is_colorable<G>(graph: &G) -> bool
    where
        G: Graph,
    {
        let mut solver = Self::graph_to_cnf_sat(graph);
        solver.solve().unwrap()
    }

    fn new() -> Self {
        SATColourizerNew {}
    }
}

impl SATColourizerNew {
    fn graph_to_cnf_sat<G: Graph>(graph: &G) -> Solver {
        let mut solver = Solver::new();
        let mut formula = CnfFormula::new();

        let mut edge_lits: HashMap<(usize, usize), (varisat::Lit, varisat::Lit, varisat::Lit)> =
            HashMap::new();

        // create edge literals
        for edge in graph.edges() {
            // xij1 - if true - given edge is coloured by color 1
            let (xij1, xij2, xij3) = solver.new_lits();
            edge_lits.insert((edge.from(), edge.to()), (xij1, xij2, xij3));
        }

        for vertex in graph.vertices() {
            let edges: Vec<&G::E> = Vec::from_iter(graph.edges_of_vertex(vertex.index()));

            // for edge in graph.edges_of_vertex(vertex.index()) {}

            // if edges.len() == 2 {
            //     // exceptions
            //     let mut exceptions = vec![];
            //     exceptions.push(vec![]);
            //
            //     let combinations = BinaryCombinationsIterator::new(6);
            //     for combination in combinations {
            //         println!("{:?}", combination)
            //
            //
            //     }
            // }

            if edges.len() == 3 {
                // exceptions
                let mut exceptions = vec![];
                exceptions.push(vec![
                    false, false, true, false, true, false, true, false, false,
                ]);
                exceptions.push(vec![
                    false, false, true, true, false, false, false, true, false,
                ]);
                exceptions.push(vec![
                    false, true, false, false, false, true, true, false, false,
                ]);
                exceptions.push(vec![
                    false, true, false, true, false, false, false, false, true,
                ]);
                exceptions.push(vec![
                    true, false, false, false, true, false, false, false, true,
                ]);
                exceptions.push(vec![
                    true, false, false, false, false, true, false, true, false,
                ]);

                let combinations = BinaryCombinationsIterator::new(9);
                // let comb = Binary
                for combination in combinations {
                    let found = exceptions
                        .iter()
                        .find(|&exception| exception == &combination);
                    if found.is_none() {
                        let mut clause = vec![];

                        let mut counter = 0;
                        for edge in edges.iter() {
                            let lits = edge_lits.get(&(edge.from(), edge.to())).unwrap();

                            if combination[counter] {
                                clause.push(!lits.0);
                            } else {
                                clause.push(lits.0);
                            }

                            if combination[counter + 1] {
                                clause.push(!lits.1);
                            } else {
                                clause.push(lits.1);
                            }

                            if combination[counter + 2] {
                                clause.push(!lits.2);
                            } else {
                                clause.push(lits.2);
                            }

                            counter += 3;
                        }

                        formula.add_clause(&clause);
                    }
                }
            }
        }

        solver.add_formula(&formula);
        solver
    }
}

struct BinaryCombinationsItemIterator {
    max_counter: usize,
    counter: usize,
    current_value: bool,
}

impl BinaryCombinationsItemIterator {
    pub fn new(max_counter: usize) -> Self {
        BinaryCombinationsItemIterator {
            max_counter,
            counter: 0,
            current_value: false,
        }
    }

    fn switch_value(&mut self) {
        if self.current_value {
            self.current_value = false;
        } else {
            self.current_value = true;
        }
    }
}

impl Iterator for BinaryCombinationsItemIterator {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;
        if self.counter > self.max_counter {
            self.counter = 1;
            self.switch_value();
        }
        Some((&self).current_value)
    }
}

struct BinaryCombinationsIterator {
    items: Vec<BinaryCombinationsItemIterator>,
    counter: usize,
    all_options_count: usize,
}

impl BinaryCombinationsIterator {
    pub fn new(size: usize) -> Self {
        let mut items = vec![];
        for item_size in (1..size + 1).rev() {
            let max_item_counter = (2 as usize).pow((item_size - 1) as u32);
            let item = BinaryCombinationsItemIterator::new(max_item_counter);
            items.push(item);
        }

        let all_options_count: usize = (2 as usize).pow(size as u32);

        BinaryCombinationsIterator {
            items,
            counter: 0,
            all_options_count,
        }
    }
}

impl Iterator for BinaryCombinationsIterator {
    type Item = Vec<bool>;

    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;
        if self.counter > self.all_options_count {
            return None;
        }
        let mut result = vec![];
        for item in self.items.iter_mut() {
            result.push(item.next().unwrap());
        }
        Some(result)
    }
}
