use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::undirected::simple_graph::graph::SimpleGraph;
use crate::service::chromatic_properties::error::ChromaticPropertiesError;
use crate::service::colour::colouriser::Colouriser;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::option::Option::Some;
use std::sync::mpsc;
use std::{result, thread};

pub type Result<T> = result::Result<T, ChromaticPropertiesError>;

pub struct EdgeSubcriticalitySolver {}

impl EdgeSubcriticalitySolver {
    pub fn compute_edge_subcriticality<C: Colouriser>(graph: &SimpleGraph) -> bool {
        let mut graph_copy = SimpleGraph::from_graph(graph);
        let mut edge_subcritical = true;

        let mut computed_pairs = HashMap::new();
        for first_edge in graph.edges() {
            graph_copy.remove_edge(first_edge.from(), first_edge.to());

            for second_edge in graph.edges() {
                if first_edge.eq(second_edge) {
                    continue;
                }
                let colourable: bool;
                if computed_pairs.contains_key(&(first_edge, second_edge)) {
                    colourable = *computed_pairs.get(&(first_edge, second_edge)).unwrap();
                } else {
                    graph_copy.remove_edge(second_edge.from(), second_edge.to());
                    colourable = C::is_colorable(&graph_copy);
                    graph_copy.add_edge(second_edge.from(), second_edge.to());

                    computed_pairs.insert((first_edge, second_edge), colourable);
                    computed_pairs.insert((second_edge, first_edge), colourable);
                }
                if colourable {
                    edge_subcritical = true;
                    break;
                }
                edge_subcritical = false;
            }
            graph_copy.add_edge(first_edge.from(), first_edge.to());
            if !edge_subcritical {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone)]
pub struct EdgeSubcriticalityParallelSolver<C>
where
    C: Colouriser + Send,
{
    pub graph: SimpleGraph,
    pub untouched_graph: SimpleGraph,
    pub computed_pairs: HashMap<(UndirectedEdge, UndirectedEdge), bool>,
    pub edge_subcritical: bool,
    _colouriser: C,
}

impl<C> EdgeSubcriticalityParallelSolver<C>
where
    C: Colouriser + Send + 'static,
{
    pub fn for_graph(graph: &SimpleGraph, colouriser: C) -> Self {
        let graph_copy = SimpleGraph::from_graph(graph);
        EdgeSubcriticalityParallelSolver {
            untouched_graph: graph_copy.clone(),
            graph: graph_copy,
            computed_pairs: HashMap::new(),
            edge_subcritical: true,
            _colouriser: colouriser,
        }
    }

    pub fn compute_edge_subcriticality_parallel(
        graph: &mut SimpleGraph,
        threads_count: usize,
    ) -> bool {
        let mut threads = HashMap::new();
        let mut index = 0;
        let (tx, rx) = mpsc::channel();

        let mut solver = EdgeSubcriticalityParallelSolver::for_graph(graph, C::new());
        let mut results_gained = false;

        let mut edge_iter = graph.edges();
        while let Some(first_edge) = edge_iter.next() {
            let mut solver_copy = solver.clone();
            let graph_ref = &mut solver_copy.graph;
            graph_ref.remove_edge(first_edge.from(), first_edge.to());

            let tx_cloned = mpsc::Sender::clone(&tx);
            let handle =
                Self::spawn_thread_for_subgraph(first_edge.clone(), tx_cloned, solver_copy, index);
            threads.insert(index, handle);
            index += 1;

            if index >= threads_count {
                break;
            }
        }

        // receive results and create new threads while next first_vertex exists
        for received in &rx {
            let received_result = received.borrow().as_ref();
            let received_result = received_result.unwrap();
            let received_index = received_result.index;

            // end/join thread which sent received results
            let thread_opt = threads.remove(&received_index);
            if thread_opt.is_some() {
                let _result = thread_opt.unwrap().join();
            }
            results_gained = solver.handle_thread_result(received_result);
            if results_gained {
                break;
            }

            let next_edge = edge_iter.next();
            if let Some(first_edge) = next_edge {
                let mut solver_copy = solver.clone();
                let graph_ref = &mut solver_copy.graph;
                graph_ref.remove_edge(first_edge.from(), first_edge.to());

                let tx_cloned = mpsc::Sender::clone(&tx);
                let handle = Self::spawn_thread_for_subgraph(
                    first_edge.clone(),
                    tx_cloned,
                    solver_copy,
                    index,
                );
                threads.insert(index, handle);
                index += 1;
            } else {
                break;
            }
        }

        drop(tx);
        //
        // // receive remaining results
        if !results_gained {
            for received in &rx {
                let received_result = received.borrow().as_ref();
                let received_result = received_result.unwrap();

                results_gained = solver.handle_thread_result(received_result);
                if results_gained {
                    break;
                }
            }
        }
        // end/join remaining threads
        for thread in threads {
            thread.1.join().unwrap();
        }
        solver.edge_subcritical
    }

    fn spawn_thread_for_subgraph(
        first_edge: UndirectedEdge,
        sender: mpsc::Sender<Result<ThreadResult<C>>>,
        solver: EdgeSubcriticalityParallelSolver<C>,
        index: usize,
    ) -> thread::JoinHandle<()> {
        let handle = thread::spawn(move || {
            let result = Self::in_thread(first_edge, solver, index);
            let result = sender.send(result);
            if result.is_err() {
                // handle otherwise?
                panic!(
                    "error while sending message between threads: {}",
                    result.err().unwrap()
                );
            }
        });
        handle
    }

    fn in_thread(
        first_edge: UndirectedEdge,
        mut solver: EdgeSubcriticalityParallelSolver<C>,
        index: usize,
    ) -> Result<ThreadResult<C>> {
        let mut edge_subcritical = false;
        let graph = &mut solver.graph;
        let computed_pairs = &mut solver.computed_pairs;

        for second_edge in solver.untouched_graph.edges() {
            if first_edge.eq(second_edge) {
                continue;
            }
            let colourable: bool;
            if computed_pairs.contains_key(&(first_edge.clone(), second_edge.clone())) {
                colourable = *computed_pairs
                    .get(&(first_edge.clone(), second_edge.clone()))
                    .unwrap();
            } else {
                graph.remove_edge(second_edge.from(), second_edge.to());
                colourable = C::is_colorable(graph);
                graph.add_edge(second_edge.from(), second_edge.to());

                computed_pairs.insert((first_edge.clone(), second_edge.clone()), colourable);
                computed_pairs.insert((second_edge.clone(), first_edge.clone()), colourable);
            }
            if colourable {
                edge_subcritical = true;
                break;
            }
        }
        Ok(ThreadResult {
            index,
            edge_subcritical,
            solver,
            first_edge: first_edge.clone(),
        })
    }

    ///
    /// returns true if gained all properties
    ///
    fn handle_thread_result(&mut self, result: &ThreadResult<C>) -> bool {
        if !result.edge_subcritical {
            self.edge_subcritical = false;
            return true;
        }
        let received_solver = &result.solver;
        let first_edge = &result.first_edge;
        let received_computed_pairs = &received_solver.computed_pairs;
        let self_computed_pairs = &mut self.computed_pairs;
        for second_edge in self.untouched_graph.edges() {
            if received_computed_pairs.contains_key(&(first_edge.clone(), second_edge.clone()))
                && !self_computed_pairs.contains_key(&(first_edge.clone(), second_edge.clone()))
            {
                let colourable = *received_computed_pairs
                    .get(&(first_edge.clone(), second_edge.clone()))
                    .unwrap();
                self_computed_pairs.insert((first_edge.clone(), second_edge.clone()), colourable);
                self_computed_pairs.insert((second_edge.clone(), first_edge.clone()), colourable);
            }
        }
        false
    }
}

struct ThreadResult<C>
where
    C: Colouriser + Send,
{
    index: usize,
    edge_subcritical: bool,
    solver: EdgeSubcriticalityParallelSolver<C>,
    first_edge: UndirectedEdge,
}
