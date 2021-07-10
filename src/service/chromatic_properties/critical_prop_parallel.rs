use crate::graph::edge::{Edge, EdgeConstructor};
use crate::graph::graph::Graph;
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::undirected::simple_graph::graph::SimpleGraph;
use crate::service::chromatic_properties::critical_prop::{
    CriticalPropertiesSolver, CriticalPropertiesStruct,
};
use crate::service::chromatic_properties::error::ChromaticPropertiesError;
use crate::service::chromatic_properties::CriticalProperties;
use crate::service::colour::colouriser::Colouriser;
use crate::service::colour::recursive::dfs_improved::DFSColourizer;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::mpsc;
use std::{result, thread};

pub type Result<T> = result::Result<T, ChromaticPropertiesError>;

#[derive(Debug, Clone)]
pub struct CriticalPropertiesParallelSolver<C>
where
    C: Colouriser + Send,
{
    _colourizer: C,
    properties: CriticalPropertiesStruct,
    threads_count: usize,
}

impl<C> CriticalProperties for CriticalPropertiesParallelSolver<C>
where
    C: Colouriser + Send + 'static,
{
    fn is_critical(&mut self) -> bool {
        if self.properties.vertex_properties_computed {
            return self.properties.is_critical;
        }
        self.compute_properties();
        return self.properties.is_critical;
    }

    fn is_cocritical(&mut self) -> bool {
        if self.properties.vertex_properties_computed {
            return self.properties.is_cocritical;
        }
        self.compute_properties();
        return self.properties.is_cocritical;
    }

    fn is_vertex_subcritical(&mut self) -> bool {
        if self.properties.vertex_properties_computed {
            return self.properties.is_vertex_subcritical;
        }
        self.compute_properties();
        return self.properties.is_vertex_subcritical;
    }

    fn is_edge_subcritical(&mut self) -> bool {
        if self.properties.edge_property_computed {
            return self.properties.is_edge_subcritical;
        }
        self.properties.is_edge_subcritical =
            Self::compute_edge_subcriticality_parallel(&mut self.properties.graph);
        self.properties.edge_property_computed = true;

        return self.properties.is_edge_subcritical;
    }

    fn is_acritical(&mut self) -> bool {
        !self.is_vertex_subcritical()
    }
}

impl<C> CriticalPropertiesParallelSolver<C>
where
    C: Colouriser + Send + 'static,
{
    pub fn of_graph_with_colourizer<G: Graph + Clone>(graph: &G, colourizer: C) -> Self {
        let cpus_count = num_cpus::get();
        CriticalPropertiesParallelSolver {
            _colourizer: colourizer,
            properties: CriticalPropertiesStruct::of_graph(graph),
            threads_count: cpus_count,
        }
    }

    pub fn with_cpus_count<G: Graph + Clone>(graph: &G, colourizer: C, cpus_count: usize) -> Self {
        CriticalPropertiesParallelSolver {
            _colourizer: colourizer,
            properties: CriticalPropertiesStruct::of_graph(graph),
            threads_count: cpus_count,
        }
    }

    fn compute_properties(&mut self) {
        self.compute_vertex_properties();

        if self.properties.is_critical {
            self.properties.is_edge_subcritical = true;
            self.properties.edge_property_computed = true;
        }
        self.properties.vertex_properties_computed = true;
    }

    ///
    /// Compute criticality, cocriticality and vertex subcriticality of graph
    ///
    fn compute_vertex_properties(&mut self) {
        self.compute_vertex_properties_parallel();
    }

    fn compute_vertex_properties_parallel(&mut self) {
        let mut threads = HashMap::new();
        let mut index = 0;
        let (tx, rx) = mpsc::channel();

        self.properties.is_critical = true;
        self.properties.is_cocritical = true;

        let graph = &mut self.properties.graph;
        let graph_size = self.properties.graph.size();

        self.properties.is_vertex_subcritical = false;
        let mut results_gained = false;

        for first_vertex in 0..graph_size {
            let mut self_copy = self.clone();
            let mut graph_copy = &mut self_copy.properties.graph;
            graph_copy.remove_edges_of_vertex(first_vertex);

            let tx_cloned = mpsc::Sender::clone(&tx);
            let handle = Self::spawn_thread_for_subgraph(self_copy, first_vertex, index, tx_cloned);
            threads.insert(index, handle);
            index += 1;

            if index >= self.threads_count {
                break;
            }
        }

        // receive results and create new threads while next first_vertex exists
        for received in &rx {
            let received_result = received.borrow().as_ref();
            let received_result = received_result.unwrap();
            let received_index = received_result.first_vertex;

            // end/join thread which sent received results
            let thread_opt = threads.remove(&received_index);
            if thread_opt.is_some() {
                let _result = thread_opt.unwrap().join();
            }
            results_gained = self.handle_thread_result(received_result, graph_size);
            if results_gained {
                break;
            }
            if index < graph_size {
                let mut self_copy = self.clone();
                let mut graph_copy = &mut self_copy.properties.graph;
                graph_copy.remove_edges_of_vertex(index);

                let tx_cloned = mpsc::Sender::clone(&tx);
                let handle = Self::spawn_thread_for_subgraph(self_copy, index, index, tx_cloned);
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

                results_gained = self.handle_thread_result(received_result, graph_size);
                if results_gained {
                    break;
                }
            }
        }
        // end/join remaining threads
        for thread in threads {
            thread.1.join().unwrap();
        }
    }

    fn spawn_thread_for_subgraph(
        mut critical_props: CriticalPropertiesParallelSolver<C>,
        first_vertex: usize,
        index: usize,
        sender: mpsc::Sender<Result<ThreadResult<C>>>,
    ) -> thread::JoinHandle<()> {
        critical_props.properties.is_vertex_subcritical = false;
        let handle = thread::spawn(move || {
            let result = Self::in_thread(critical_props, first_vertex);
            let result = sender.send(result);
            if result.is_err() {
                // TODO - handle somehow?
                eprintln!(
                    "error while sending message between threads: {}",
                    result.err().unwrap()
                );
            }
        });
        handle
    }

    ///
    /// returns true if gained all properties
    ///
    fn handle_thread_result(&mut self, result: &ThreadResult<C>, graph_size: usize) -> bool {
        let received_props = &result.props;
        let received_index = result.first_vertex;
        if !received_props.properties.is_critical {
            self.properties.is_critical = false;
        }
        if !received_props.properties.is_cocritical {
            self.properties.is_cocritical = false;
        }
        if !received_props.properties.is_vertex_subcritical {
            self.properties.is_vertex_subcritical = false;
            return true;
        } else {
            self.properties.is_vertex_subcritical = true;
        }

        for column in 0..graph_size {
            self.properties.colourings[received_index * graph_size + column] =
                received_props.properties.colourings[received_index * graph_size + column];
            self.properties.colourings[column * graph_size + received_index] =
                received_props.properties.colourings[column * graph_size + received_index];
        }
        false
    }

    // fn in_thread(&mut self, first_vertex: usize) {
    // fn in_thread(mut props: CriticalProperties<C>, first_vertex: usize) -> Result<CriticalProperties<C>> {
    fn in_thread(
        mut props: CriticalPropertiesParallelSolver<C>,
        first_vertex: usize,
    ) -> Result<ThreadResult<C>> {
        let graph = &mut props.properties.graph;

        // do in thread
        for second_vertex in 0..graph.size() {
            if first_vertex == second_vertex {
                continue;
            }

            // skip unnecessary tests
            if props
                .properties
                .untouched_graph
                .has_edge(first_vertex, second_vertex)
                && !props.properties.is_critical
                && props.properties.is_vertex_subcritical
            {
                continue;
            }
            if !props
                .properties
                .untouched_graph
                .has_edge(first_vertex, second_vertex)
                && !props.properties.is_cocritical
                && props.properties.is_vertex_subcritical
            {
                continue;
            }

            let colourable_opt =
                props.properties.colourings[first_vertex * graph.size() + second_vertex];
            let colourable;
            if colourable_opt.is_some() {
                colourable = colourable_opt.unwrap();
            } else {
                graph.remove_edges_of_vertex(second_vertex);

                colourable = C::is_colorable(graph);

                props.properties.colourings[first_vertex * graph.size() + second_vertex] =
                    Some(colourable);
                props.properties.colourings[second_vertex * graph.size() + first_vertex] =
                    Some(colourable);

                restore_edges_of_vertex_except_for(
                    &props.properties.untouched_graph,
                    graph,
                    second_vertex,
                    first_vertex,
                );
            }

            // check properties
            if !colourable {
                if props
                    .properties
                    .untouched_graph
                    .has_edge(first_vertex, second_vertex)
                {
                    props.properties.is_critical = false;
                } else {
                    props.properties.is_cocritical = false;
                }
            } else {
                props.properties.is_vertex_subcritical = true;
            }
        }

        Ok(ThreadResult {
            props,
            first_vertex,
        })
    }

    // TODO - make parallel
    pub fn compute_edge_subcriticality_parallel(graph: &mut SimpleGraph) -> bool {
        let local_graph = SimpleGraph::from_graph(graph);
        let mut edge_subcritical = true;

        // TODO - optimization
        // TODO - avoid computing for repeating pairs - pair (0, 1), (3, 4) is the same as (3, 4), (0, 1)

        for first_edge in local_graph.edges() {
            graph.remove_edge(first_edge.from(), first_edge.to());

            // TODO - spawn thread
            // do in thread

            for second_edge in local_graph.edges() {
                if first_edge.eq(second_edge) {
                    continue;
                }
                graph.remove_edge(second_edge.from(), second_edge.to());
                let colourable = C::is_colorable(graph);
                graph.add_edge(second_edge.from(), second_edge.to());
                if colourable {
                    edge_subcritical = true;
                    break;
                }
                edge_subcritical = false;
            }
            graph.add_edge(first_edge.from(), first_edge.to());
            if !edge_subcritical {
                return false;
            }
        }
        true
    }

    pub fn set_cpus_count(&mut self, cpus_count: usize) {
        self.threads_count = cpus_count;
    }
}

impl CriticalPropertiesParallelSolver<DFSColourizer> {
    #[allow(dead_code)]
    pub fn of_graph<G: Graph + Clone>(graph: &G) -> Self {
        CriticalPropertiesParallelSolver::<DFSColourizer>::of_graph_with_colourizer(
            graph,
            DFSColourizer::new(),
        )
    }
}

pub fn restore_edges_of_vertex_except_for(
    original_graph: &SimpleGraph,
    changed_graph: &mut SimpleGraph,
    vertex: usize,
    except_for: usize,
) {
    let except_for_edge = UndirectedEdge::new(vertex, except_for);
    for neighboring_edge in original_graph.vertices[vertex].edges.iter() {
        if neighboring_edge.from() == except_for_edge.from()
            && neighboring_edge.to() == except_for_edge.to()
        {
            continue;
        }
        changed_graph.add_edge(neighboring_edge.from(), neighboring_edge.to());
    }
}

struct ThreadResult<C: Colouriser + Send> {
    props: CriticalPropertiesParallelSolver<C>,
    first_vertex: usize,
}
