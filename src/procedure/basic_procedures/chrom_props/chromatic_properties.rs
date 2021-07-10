use crate::graph::undirected::simple_graph::graph::SimpleGraph;
use crate::graph::undirected::UndirectedGraph;
use crate::procedure::basic_procedures::chrom_props::config::{
    ChromaticPropertiesToCompute, ChromaticPropsProcedureConfig, ParallelizationType, ACRITICAL,
    COCRITICAL, COSTABLE, CRITICAL, CYCLIC_EDGE_CONNECTIVITY, EDGE_RESISTIBILITIES,
    EDGE_RESISTIBILITY_INDEX, EDGE_SUBCRITICAL, GIRTH, GRAPH_INDEX, ODDNESS, RESISTANCE, STABLE,
    VERTEX_RESISTIBILITIES, VERTEX_RESISTIBILITY_INDEX, VERTEX_SUBCRITICAL,
};
use crate::procedure::basic_procedures::colour::ColouriserType;
use crate::procedure::helpers::config_helper;
use crate::procedure::helpers::serialize_helper;
use crate::procedure::procedure;
use crate::procedure::procedure::{GraphProperties, Procedure};
use crate::procedure::procedure_builder::{Config, ProcedureBuilder};
use crate::service::chromatic_properties::critical_prop::CriticalPropertiesSolver;
use crate::service::chromatic_properties::critical_prop_parallel::CriticalPropertiesParallelSolver;
use crate::service::chromatic_properties::error::ChromaticPropertiesError;
use crate::service::chromatic_properties::resistance::Resistance;
use crate::service::chromatic_properties::resistibility::Resistibility;
use crate::service::chromatic_properties::stable_and_critical_prop::StableAndCriticalProperties;
use crate::service::chromatic_properties::CriticalProperties;
use crate::service::colour::colouriser::Colouriser;
use crate::service::colour::recursive::dfs_improved::DFSColourizer;
use crate::service::colour::sat::sat::SATColourizer;
use crate::service::property::cyclic_connectivity::cyclic_edge_connectivity;
use crate::service::property::girth::girth;
use crate::service::property::oddness::Oddness;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::mpsc;
use std::{marker, result, thread};

pub type Result<T> = result::Result<T, ChromaticPropertiesError>;

struct ChromaticPropsProcedure<G> {
    config: ChromaticPropsProcedureConfig,
    _ph: marker::PhantomData<G>,
}

pub struct ChromaticPropsProcedureBuilder {}

impl<G: UndirectedGraph + Clone> Procedure<G> for ChromaticPropsProcedure<G> {
    fn run(&self, graphs: &mut Vec<(G, GraphProperties)>) -> procedure::Result<()> {
        println!("running chromatic properties procedure");
        self.chromatic_properties(graphs)?;
        Ok(())
    }
}

impl<G: UndirectedGraph + Clone> ChromaticPropsProcedure<G> {
    fn chromatic_properties(&self, graphs: &mut Vec<(G, GraphProperties)>) -> Result<()> {
        // let parallel = self.config.parallel();
        let colouriser_type = self.config.colouriser_type();
        // if parallel {
        //     self.chromatic_properties_parallel(graphs, colouriser_type)?;
        // } else {
        //     self.chromatic_properties_sequential(graphs, colouriser_type)?;
        // }

        let parallelization = self.config.parallelization();
        match parallelization {
            ParallelizationType::BatchBased => {
                self.chromatic_properties_batch_parallel(graphs, colouriser_type)?;
            }
            ParallelizationType::GraphBased => {
                self.chromatic_properties_graph_parallel(graphs, colouriser_type)?;
            }
            ParallelizationType::None => {
                self.chromatic_properties_sequential(graphs, colouriser_type)?;
            }
        }
        Ok(())
    }

    fn chromatic_properties_batch_parallel(
        &self,
        graphs: &mut Vec<(G, GraphProperties)>,
        colouriser_type: &ColouriserType,
    ) -> Result<()> {
        let mut threads = HashMap::new();
        let mut index = 0;
        let (tx, rx) = mpsc::channel();

        let cpus_count = self.config.max_threads;
        let to_compute = &self.config.properties_to_compute;

        // init first threads
        let mut graphs_iter = graphs.iter();
        let mut next_graph = graphs_iter.next();
        while next_graph.is_some() {
            let graph = next_graph.unwrap();
            // if graph is bigger could cause performance issues
            let graph_local = SimpleGraph::from_graph(&graph.0);
            let tx_cloned = mpsc::Sender::clone(&tx);
            let handle = Self::spawn_thread_for_graph(
                graph_local,
                index,
                (*colouriser_type).clone(),
                (*to_compute).clone(),
                tx_cloned,
            );
            threads.insert(index, handle);
            index += 1;
            if index >= cpus_count {
                break;
            }
            next_graph = graphs_iter.next();
        }
        let mut results = Vec::with_capacity(graphs.len());

        // receive results and create new threads while next graphs exists
        for received in &rx {
            let received_result = received.borrow().as_ref();
            let index_value = received_result.unwrap().get(GRAPH_INDEX).unwrap();
            let received_index: usize = serde_json::from_value(index_value.clone())?;

            // end/join thread which sent received results
            let thread_opt = threads.remove(&received_index);
            if thread_opt.is_some() {
                let _result = thread_opt.unwrap().join();
            }
            results.push(received);

            next_graph = graphs_iter.next();
            if next_graph.is_some() {
                let graph = next_graph.unwrap();
                let graph_local = SimpleGraph::from_graph(&graph.0);
                let tx_cloned = mpsc::Sender::clone(&tx);
                let handle = Self::spawn_thread_for_graph(
                    graph_local,
                    index,
                    colouriser_type.clone(),
                    (*to_compute).clone(),
                    tx_cloned,
                );
                threads.insert(index, handle);
                index += 1;
            } else {
                break;
            }
        }

        drop(tx);

        // receive remaining results
        for received in rx {
            results.push(received);
        }
        for result in results {
            self.handle_parallel_result(graphs, result)?;
        }
        // end/join remaining threads
        for thread in threads {
            thread.1.join().unwrap();
        }
        Ok(())
    }

    fn chromatic_properties_graph_parallel(
        &self,
        graphs: &mut Vec<(G, GraphProperties)>,
        colouriser_type: &ColouriserType,
    ) -> Result<()> {
        // TODO - rewrite for graph parallelism
        // foreach property to compute - spawn onw thread?
        // in case of critical and stable props - in separate thread spawn another threads
        //

        let mut index = 0;
        for graph in graphs {
            let properties = Self::compute_properties_by_colouriser_parallel(
                &graph.0,
                colouriser_type,
                index,
                &self.config.properties_to_compute,
                self.config.max_threads
            )?;
            self.write_properties(graph, properties)?;
            index += 1;
        }
        Ok(())

        // unimplemented!()

        // let mut threads = HashMap::new();
        // let mut index = 0;
        // let (tx, rx) = mpsc::channel();
        //
        // // TODO - do not user num_cpus directly - use as configurable param with default
        // //  num_cpus::get()
        // let cpus_count = num_cpus::get();
        //
        // let to_compute = &self.config.properties_to_compute;
        //
        // // init first threads
        // let mut graphs_iter = graphs.iter();
        // let mut next_graph = graphs_iter.next();
        // while next_graph.is_some() {
        //     let graph = next_graph.unwrap();
        //     // if graph is bigger could cause performance issues
        //     let graph_local = SimpleGraph::from_graph(&graph.0);
        //     let tx_cloned = mpsc::Sender::clone(&tx);
        //     let handle = Self::spawn_thread_for_graph(
        //         graph_local,
        //         index,
        //         (*colouriser_type).clone(),
        //         (*to_compute).clone(),
        //         tx_cloned,
        //     );
        //     threads.insert(index, handle);
        //     index += 1;
        //     if index >= cpus_count {
        //         break;
        //     }
        //     next_graph = graphs_iter.next();
        // }
        // let mut results = Vec::with_capacity(graphs.len());
        //
        // // receive results and create new threads while next graphs exists
        // for received in &rx {
        //     let received_result = received.borrow().as_ref();
        //     let index_value = received_result.unwrap().get(GRAPH_INDEX).unwrap();
        //     let received_index: usize = serde_json::from_value(index_value.clone())?;
        //
        //     // end/join thread which sent received results
        //     let thread_opt = threads.remove(&received_index);
        //     if thread_opt.is_some() {
        //         let _result = thread_opt.unwrap().join();
        //     }
        //     results.push(received);
        //
        //     next_graph = graphs_iter.next();
        //     if next_graph.is_some() {
        //         let graph = next_graph.unwrap();
        //         let graph_local = SimpleGraph::from_graph(&graph.0);
        //         let tx_cloned = mpsc::Sender::clone(&tx);
        //         let handle = Self::spawn_thread_for_graph(
        //             graph_local,
        //             index,
        //             colouriser_type.clone(),
        //             (*to_compute).clone(),
        //             tx_cloned,
        //         );
        //         threads.insert(index, handle);
        //         index += 1;
        //     } else {
        //         break;
        //     }
        // }
        //
        // drop(tx);
        //
        // // receive remaining results
        // for received in rx {
        //     results.push(received);
        // }
        // for result in results {
        //     self.handle_parallel_result(graphs, result)?;
        // }
        // // end/join remaining threads
        // for thread in threads {
        //     thread.1.join().unwrap();
        // }
        // Ok(())
    }

    fn handle_parallel_result(
        &self,
        graphs: &mut Vec<(G, GraphProperties)>,
        result: Result<GraphProperties>,
    ) -> Result<()> {
        let result_props = result?;
        let graph_index_opt_value = result_props.get(GRAPH_INDEX);
        if graph_index_opt_value.is_some() {
            let graph_index_value = graph_index_opt_value.unwrap();
            let graph_index: usize = serde_json::from_value(graph_index_value.clone())?;
            graphs[graph_index].1.extend(result_props);
            return Ok(());
        }
        Err(ChromaticPropertiesError::new(
            "graph index is missing in parallel result",
        ))
    }

    fn spawn_thread_for_graph(
        graph: SimpleGraph,
        index: usize,
        colouriser_type: ColouriserType,
        properties_to_compute: ChromaticPropertiesToCompute,
        sender: mpsc::Sender<Result<GraphProperties>>,
    ) -> thread::JoinHandle<()> {
        let handle = thread::spawn(move || {
            let result = Self::compute_properties_by_colouriser(
                &graph,
                &colouriser_type,
                index,
                &properties_to_compute,
            );
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

    fn chromatic_properties_sequential(
        &self,
        graphs: &mut Vec<(G, GraphProperties)>,
        colouriser_type: &ColouriserType,
    ) -> Result<()> {
        let mut index = 0;
        for graph in graphs {
            let properties = Self::compute_properties_by_colouriser(
                &graph.0,
                colouriser_type,
                index,
                &self.config.properties_to_compute,
            )?;
            self.write_properties(graph, properties)?;
            index += 1;
        }
        Ok(())
    }

    fn compute_properties_by_colouriser<Gr: UndirectedGraph + Clone>(
        graph: &Gr,
        colouriser_type: &ColouriserType,
        graph_index: usize,
        properties_to_compute: &ChromaticPropertiesToCompute,
    ) -> Result<GraphProperties> {
        // to do - change colouriser type according to graph size ...
        match colouriser_type {
            ColouriserType::Sat => {
                return Self::compute_properties(
                    graph,
                    SATColourizer::new(),
                    graph_index,
                    &properties_to_compute,
                );
            }
            ColouriserType::Dfs => {
                return Self::compute_properties(
                    graph,
                    DFSColourizer::new(),
                    graph_index,
                    &properties_to_compute,
                );
            }
            _ => {
                return Err(ChromaticPropertiesError {
                    message: format!("unknown colourizer to compute chromatic properties"),
                });
            }
        }
    }

    // TODO - dedup
    fn compute_properties_by_colouriser_parallel<Gr: UndirectedGraph + Clone>(
        graph: &Gr,
        colouriser_type: &ColouriserType,
        graph_index: usize,
        properties_to_compute: &ChromaticPropertiesToCompute,
        max_threads: usize
    ) -> Result<GraphProperties> {
        // to do - change colouriser type according to graph size ...
        match colouriser_type {
            ColouriserType::Sat => {
                return Self::compute_properties_parallel(
                    graph,
                    SATColourizer::new(),
                    graph_index,
                    &properties_to_compute,
                    max_threads
                );
            }
            ColouriserType::Dfs => {
                return Self::compute_properties_parallel(
                    graph,
                    DFSColourizer::new(),
                    graph_index,
                    &properties_to_compute,
                    max_threads
                );
            }
            _ => {
                return Err(ChromaticPropertiesError {
                    message: format!("unknown colourizer to compute chromatic properties"),
                });
            }
        }
    }

    fn compute_properties<Gr: UndirectedGraph + Clone, C: Colouriser>(
        graph: &Gr,
        colouriser: C,
        graph_index: usize,
        properties_to_compute: &ChromaticPropertiesToCompute,
    ) -> Result<GraphProperties> {
        let to_compute = properties_to_compute;
        let mut properties = GraphProperties::new();
        properties.insert(GRAPH_INDEX.to_string(), serde_json::to_value(graph_index)?);

        if to_compute.stable || to_compute.costable {
            Self::critical_and_stable_properties(
                graph,
                &colouriser,
                properties_to_compute,
                &mut properties,
            )?;
        } else if to_compute.critical
            || to_compute.cocritical
            || to_compute.vertex_subcritical
            || to_compute.edge_subcritical
        {
            // compute critical props
            Self::critical_properties(graph, &colouriser, properties_to_compute, &mut properties)?;
        }

        if to_compute.resistance {
            // compute resistence and add result to properties
            Self::resistance(graph, &colouriser, &mut properties)?;
        }
        if to_compute.vertex_resistibility {
            // compute vertex resistibility and add result to properties
            Self::vertex_resistibility(graph, &colouriser, &mut properties)?;
        }
        if to_compute.edge_resistibility {
            // compute edge resistibility and add result to properties
            Self::edge_resistibility(graph, &colouriser, &mut properties)?;
        }
        if to_compute.girth {
            // compute girth and add result to properties
            let girth = girth(graph);
            properties.insert(GIRTH.to_string(), serde_json::to_value(girth)?);
        }
        if to_compute.cyclic_connectivity {
            // compute cyclic connectivity and add result to properties
            let cyclic_edge_connectivity = cyclic_edge_connectivity(graph);
            properties.insert(
                CYCLIC_EDGE_CONNECTIVITY.to_string(),
                serde_json::to_value(cyclic_edge_connectivity)?,
            );
        }
        if to_compute.oddness {
            // compute cyclic connectivity and add result to properties
            let oddness = Oddness::of_graph(graph);
            properties.insert(ODDNESS.to_string(), serde_json::to_value(oddness)?);
        }

        Ok(properties)
    }

    // TODO - dedup
    fn compute_properties_parallel<Gr: UndirectedGraph + Clone, C: Colouriser + Send + 'static>(
        graph: &Gr,
        colouriser: C,
        graph_index: usize,
        properties_to_compute: &ChromaticPropertiesToCompute,
        max_threads: usize
    ) -> Result<GraphProperties> {
        let to_compute = properties_to_compute;
        let mut properties = GraphProperties::new();
        properties.insert(GRAPH_INDEX.to_string(), serde_json::to_value(graph_index)?);

        // TODO - spawn thread foreach property - and solve each property independently

        if to_compute.stable || to_compute.costable {
            Self::critical_and_stable_properties(
                graph,
                &colouriser,
                properties_to_compute,
                &mut properties,
            )?;
        } else if to_compute.critical
            || to_compute.cocritical
            || to_compute.vertex_subcritical
            || to_compute.edge_subcritical
        {
            // compute critical props
            Self::critical_properties_parallel(
                graph,
                &colouriser,
                properties_to_compute,
                &mut properties,
                max_threads
            )?;
        }

        if to_compute.resistance {
            // compute resistence and add result to properties
            Self::resistance(graph, &colouriser, &mut properties)?;
        }
        if to_compute.vertex_resistibility {
            // compute vertex resistibility and add result to properties
            Self::vertex_resistibility(graph, &colouriser, &mut properties)?;
        }
        if to_compute.edge_resistibility {
            // compute edge resistibility and add result to properties
            Self::edge_resistibility(graph, &colouriser, &mut properties)?;
        }
        if to_compute.girth {
            // compute girth and add result to properties
            let girth = girth(graph);
            properties.insert(GIRTH.to_string(), serde_json::to_value(girth)?);
        }
        if to_compute.cyclic_connectivity {
            // compute cyclic connectivity and add result to properties
            let cyclic_edge_connectivity = cyclic_edge_connectivity(graph);
            properties.insert(
                CYCLIC_EDGE_CONNECTIVITY.to_string(),
                serde_json::to_value(cyclic_edge_connectivity)?,
            );
        }
        if to_compute.oddness {
            // compute cyclic connectivity and add result to properties
            let oddness = Oddness::of_graph(graph);
            properties.insert(ODDNESS.to_string(), serde_json::to_value(oddness)?);
        }

        Ok(properties)
    }

    // TODO - create parallel version
    fn critical_and_stable_properties<Gr: UndirectedGraph + Clone, C: Colouriser>(
        graph: &Gr,
        _colouriser: &C,
        properties_to_compute: &ChromaticPropertiesToCompute,
        properties_computed: &mut GraphProperties,
    ) -> Result<()> {
        let mut props = StableAndCriticalProperties::of_graph_with_colourizer(graph, C::new());
        Self::add_critical_properties(properties_to_compute, properties_computed, &mut props);
        if properties_to_compute.stable {
            properties_computed.insert(
                STABLE.to_string(),
                serde_json::Value::Bool(props.is_stable()),
            );
        }
        if properties_to_compute.costable {
            properties_computed.insert(
                COSTABLE.to_string(),
                serde_json::Value::Bool(props.is_costable()),
            );
        }
        Ok(())
    }

    fn critical_properties<Gr: UndirectedGraph + Clone, C: Colouriser>(
        graph: &Gr,
        _colouriser: &C,
        properties_to_compute: &ChromaticPropertiesToCompute,
        properties_computed: &mut GraphProperties,
    ) -> Result<()> {
        let mut props = CriticalPropertiesSolver::of_graph_with_colourizer(graph, C::new());
        Self::add_critical_properties(properties_to_compute, properties_computed, &mut props)
    }

    fn critical_properties_parallel<Gr: UndirectedGraph + Clone, C: Colouriser + Send + 'static>(
        graph: &Gr,
        _colouriser: &C,
        properties_to_compute: &ChromaticPropertiesToCompute,
        properties_computed: &mut GraphProperties,
        max_threads: usize
    ) -> Result<()> {
        let mut props = CriticalPropertiesParallelSolver::of_graph_with_colourizer(graph, C::new());
        props.set_cpus_count(max_threads);
        Self::add_critical_properties(properties_to_compute, properties_computed, &mut props)
    }

    fn add_critical_properties(
        properties_to_compute: &ChromaticPropertiesToCompute,
        properties_computed: &mut GraphProperties,
        props: &mut impl CriticalProperties,
    ) -> Result<()> {
        if properties_to_compute.critical {
            properties_computed.insert(
                CRITICAL.to_string(),
                serde_json::Value::Bool(props.is_critical()),
            );
        }
        if properties_to_compute.cocritical {
            properties_computed.insert(
                COCRITICAL.to_string(),
                serde_json::Value::Bool(props.is_cocritical()),
            );
        }
        if properties_to_compute.vertex_subcritical {
            properties_computed.insert(
                VERTEX_SUBCRITICAL.to_string(),
                serde_json::Value::Bool(props.is_vertex_subcritical()),
            );
        }
        if properties_to_compute.edge_subcritical {
            properties_computed.insert(
                EDGE_SUBCRITICAL.to_string(),
                serde_json::Value::Bool(props.is_edge_subcritical()),
            );
        }
        if properties_to_compute.acritical {
            properties_computed.insert(
                ACRITICAL.to_string(),
                serde_json::Value::Bool(props.is_acritical()),
            );
        }
        Ok(())
    }

    fn resistance<Gr: UndirectedGraph + Clone, C: Colouriser>(
        graph: &Gr,
        _colouriser: &C,
        properties_computed: &mut GraphProperties,
    ) -> Result<()> {
        let resistance = Resistance::new_with_colouriser(C::new());
        let resistance = resistance.vertex_resistance(graph);
        if resistance.is_some() {
            properties_computed.insert(
                RESISTANCE.to_string(),
                serde_json::to_value(resistance.unwrap())?,
            );
        } else {
            properties_computed.insert(
                RESISTANCE.to_string(),
                serde_json::Value::String("None".to_string()),
            );
        }
        Ok(())
    }

    fn edge_resistibility<Gr: UndirectedGraph + Clone, C: Colouriser>(
        graph: &Gr,
        _colouriser: &C,
        properties_computed: &mut GraphProperties,
    ) -> Result<()> {
        let mut resistibility = Resistibility::of_graph_with_colouriser(graph, C::new());
        let edge_resistibilities = resistibility.edges_resistibility();
        let edge_resistibilities_json = serialize_helper::map_to_json_value(edge_resistibilities)?;
        properties_computed.insert(EDGE_RESISTIBILITIES.to_string(), edge_resistibilities_json);

        let index_of_edge_resistibility = resistibility.edge_resistibility_index();
        properties_computed.insert(
            EDGE_RESISTIBILITY_INDEX.to_string(),
            serde_json::to_value(index_of_edge_resistibility)?,
        );

        Ok(())
    }

    fn vertex_resistibility<Gr: UndirectedGraph + Clone, C: Colouriser>(
        graph: &Gr,
        _colouriser: &C,
        properties_computed: &mut GraphProperties,
    ) -> Result<()> {
        let mut resistibility = Resistibility::of_graph_with_colouriser(graph, C::new());
        let vertices_resistibility = resistibility.vertices_resistibility();
        // let vertex_resistibilities_json =
        //     serialize_helper::vec_to_json_value(vertices_resistibility)?;
        let vertex_resistibilities_json = serde_json::to_value(vertices_resistibility)?;
        properties_computed.insert(
            VERTEX_RESISTIBILITIES.to_string(),
            vertex_resistibilities_json,
        );

        let vertex_resistibility_index = resistibility.vertex_resistibility_index();
        properties_computed.insert(
            VERTEX_RESISTIBILITY_INDEX.to_string(),
            serde_json::to_value(vertex_resistibility_index)?,
        );

        Ok(())
    }

    fn write_properties(
        &self,
        graph: &mut (G, GraphProperties),
        props: GraphProperties,
    ) -> Result<()> {
        graph.1.extend(props);
        Ok(())
    }
}

impl<G: UndirectedGraph + Clone + 'static> ProcedureBuilder<G> for ChromaticPropsProcedureBuilder {
    fn build(&self, config: Config) -> procedure::Result<Box<dyn Procedure<G>>> {
        let proc_config = ChromaticPropsProcedureConfig::from_proc_config(&config)?;
        Ok(Box::new(ChromaticPropsProcedure {
            config: proc_config,
            _ph: marker::PhantomData,
        }))
    }
}
