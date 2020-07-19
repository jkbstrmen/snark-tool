use crate::graph::graph::Graph;
use crate::graph::undirected::simple_graph::SimpleGraph;
use crate::procedure::helpers::config_helper;
use crate::procedure::helpers::serialize_helper;
use crate::procedure::procedure;
use crate::procedure::procedure::{GraphProperties, Procedure};
use crate::procedure::procedure_builder::{Config, ProcedureBuilder};
use crate::service::chromatic_properties::critical_prop::CriticalProperties;
use crate::service::chromatic_properties::error::ChromaticPropertiesError;
use crate::service::chromatic_properties::resistance::Resistance;
use crate::service::chromatic_properties::resistibility::Resistibility;
use crate::service::chromatic_properties::stable_and_critical_prop::StableAndCriticalProperties;
use crate::service::colour::bfs::BFSColourizer;
use crate::service::colour::colouriser::Colourizer;
use crate::service::colour::sat::SATColourizer;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::mpsc;
use std::{marker, result, thread};

pub type Result<T> = result::Result<T, ChromaticPropertiesError>;

struct ChromaticPropsProcedure<G> {
    config: ChromaticPropsProcedureConfig,
    _ph: marker::PhantomData<G>,
}

struct ChromaticPropsProcedureConfig {
    colouriser_type: String,
    parallel: bool,
    properties_to_compute: ChromaticPropertiesToCompute,
}

pub struct ChromaticPropsProcedureBuilder {}

#[derive(Clone)]
struct ChromaticPropertiesToCompute {
    critical: bool,
    cocritical: bool,
    vertex_subcritical: bool,
    edge_subcritical: bool,
    acritical: bool,
    stable: bool,
    costable: bool,

    resistance: bool,
    edge_resistibility: bool,
    vertex_resistibility: bool,
    girth: bool,
    cyclic_connectivity: bool,
}

impl ChromaticPropertiesToCompute {
    pub fn new() -> Self {
        ChromaticPropertiesToCompute {
            critical: false,
            cocritical: false,
            vertex_subcritical: false,
            edge_subcritical: false,
            acritical: false,
            stable: false,
            costable: false,
            resistance: false,
            edge_resistibility: false,
            vertex_resistibility: false,
            girth: false,
            cyclic_connectivity: false,
        }
    }
}

impl<G: Graph + Clone> Procedure<G> for ChromaticPropsProcedure<G> {
    fn run(&self, graphs: &mut Vec<(G, GraphProperties)>) -> procedure::Result<()> {
        println!("running chromatic properties procedure");
        self.chromatic_properties(graphs)?;
        Ok(())
    }
}

impl<G: Graph + Clone> ChromaticPropsProcedure<G> {
    fn chromatic_properties(&self, graphs: &mut Vec<(G, GraphProperties)>) -> Result<()> {
        let parallel = self.config.parallel();
        let colouriser_type = self.config.colouriser_type();
        if parallel {
            self.chromatic_properties_parallel(graphs, colouriser_type)?;
        } else {
            self.chromatic_properties_sequential(graphs, colouriser_type)?;
        }
        Ok(())
    }

    fn chromatic_properties_parallel(
        &self,
        graphs: &mut Vec<(G, GraphProperties)>,
        colouriser_type: &String,
    ) -> Result<()> {
        let mut threads = HashMap::new();
        let mut index = 0;
        let (tx, rx) = mpsc::channel();
        let cpus_count = num_cpus::get();

        let to_compute = &self.config.properties_to_compute;

        // init first threads
        let mut graphs_iter = graphs.iter();
        let mut next_graph = graphs_iter.next();
        while next_graph.is_some() {
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
            if index >= cpus_count {
                break;
            }
            next_graph = graphs_iter.next();
        }
        let mut results = Vec::with_capacity(graphs.len());

        // receive results and create new threads while next graphs exists
        for received in &rx {
            let received_result = received.borrow().as_ref();
            let index_value = received_result.unwrap().get("graph_index").unwrap();
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

    fn handle_parallel_result(
        &self,
        graphs: &mut Vec<(G, GraphProperties)>,
        result: Result<GraphProperties>,
    ) -> Result<()> {
        let result_props = result?;
        let graph_index_opt_value = result_props.get("graph_index");
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
        colouriser_type: String,
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
        colouriser_type: &String,
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

    fn compute_properties_by_colouriser<Gr: Graph + Clone>(
        graph: &Gr,
        colouriser_type: &String,
        graph_index: usize,
        properties_to_compute: &ChromaticPropertiesToCompute,
    ) -> Result<GraphProperties> {
        // to do - change colouriser type according to graph size ...
        match colouriser_type.as_str() {
            "sat" => {
                return Self::compute_properties(
                    graph,
                    SATColourizer::new(),
                    graph_index,
                    &properties_to_compute,
                );
            }
            "bfs" => {
                return Self::compute_properties(
                    graph,
                    BFSColourizer::new(),
                    graph_index,
                    &properties_to_compute,
                );
            }
            _ => {
                return Err(ChromaticPropertiesError {
                    message: format!(
                        "unknown colourizer: {} for chromatic properties",
                        colouriser_type
                    ),
                });
            }
        }
    }

    fn compute_properties<Gr: Graph + Clone, C: Colourizer>(
        graph: &Gr,
        colouriser: C,
        graph_index: usize,
        properties_to_compute: &ChromaticPropertiesToCompute,
    ) -> Result<GraphProperties> {
        let to_compute = properties_to_compute;
        let mut properties = GraphProperties::new();
        properties.insert(
            "graph_index".to_string(),
            serde_json::to_value(graph_index)?,
        );

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
        }
        if to_compute.cyclic_connectivity {
            // compute cyclic connectivity and add result to properties
        }

        Ok(properties)
    }

    fn critical_and_stable_properties<Gr: Graph + Clone, C: Colourizer>(
        graph: &Gr,
        _colouriser: &C,
        properties_to_compute: &ChromaticPropertiesToCompute,
        properties_computed: &mut GraphProperties,
    ) -> Result<()> {
        let mut props = StableAndCriticalProperties::of_graph_with_colourizer(graph, C::new());
        if properties_to_compute.critical {
            properties_computed.insert(
                "critical".to_string(),
                serde_json::Value::Bool(props.is_critical()),
            );
        }
        if properties_to_compute.cocritical {
            properties_computed.insert(
                "cocritical".to_string(),
                serde_json::Value::Bool(props.is_cocritical()),
            );
        }
        if properties_to_compute.vertex_subcritical {
            properties_computed.insert(
                "vertex_subcritical".to_string(),
                serde_json::Value::Bool(props.is_vertex_subcritical()),
            );
        }
        if properties_to_compute.edge_subcritical {
            properties_computed.insert(
                "edge_subcritical".to_string(),
                serde_json::Value::Bool(props.is_edge_subcritical()),
            );
        }
        if properties_to_compute.stable {
            properties_computed.insert(
                "stable".to_string(),
                serde_json::Value::Bool(props.is_stable()),
            );
        }
        if properties_to_compute.costable {
            properties_computed.insert(
                "costable".to_string(),
                serde_json::Value::Bool(props.is_costable()),
            );
        }
        Ok(())
    }

    fn critical_properties<Gr: Graph + Clone, C: Colourizer>(
        graph: &Gr,
        _colouriser: &C,
        properties_to_compute: &ChromaticPropertiesToCompute,
        properties_computed: &mut GraphProperties,
    ) -> Result<()> {
        let mut props = CriticalProperties::of_graph_with_colourizer(graph, C::new());
        if properties_to_compute.critical {
            properties_computed.insert(
                "critical".to_string(),
                serde_json::Value::Bool(props.is_critical()),
            );
        }
        if properties_to_compute.cocritical {
            properties_computed.insert(
                "cocritical".to_string(),
                serde_json::Value::Bool(props.is_cocritical()),
            );
        }
        if properties_to_compute.vertex_subcritical {
            properties_computed.insert(
                "vertex_subcritical".to_string(),
                serde_json::Value::Bool(props.is_vertex_subcritical()),
            );
        }
        if properties_to_compute.edge_subcritical {
            properties_computed.insert(
                "edge_subcritical".to_string(),
                serde_json::Value::Bool(props.is_edge_subcritical()),
            );
        }
        Ok(())
    }

    fn resistance<Gr: Graph + Clone, C: Colourizer>(
        graph: &Gr,
        _colouriser: &C,
        properties_computed: &mut GraphProperties,
    ) -> Result<()> {
        let resistance = Resistance::new_with_colouriser(C::new());
        let resistance = resistance.vertex_resistance(graph);
        if resistance.is_some() {
            properties_computed.insert(
                "resistance".to_string(),
                serde_json::to_value(resistance.unwrap())?,
            );
        } else {
            properties_computed.insert(
                "resistance".to_string(),
                serde_json::Value::String("None".to_string()),
            );
        }
        Ok(())
    }

    fn edge_resistibility<Gr: Graph + Clone, C: Colourizer>(
        graph: &Gr,
        _colouriser: &C,
        properties_computed: &mut GraphProperties,
    ) -> Result<()> {
        let mut resistibility = Resistibility::of_graph_with_colouriser(graph, C::new());
        let edge_resistibilities = resistibility.edges_resistibility();
        let edge_resistibilities_json = serialize_helper::map_to_json_value(edge_resistibilities)?;
        properties_computed.insert(
            "edge_resistibilities".to_string(),
            edge_resistibilities_json,
        );

        let index_of_edge_resistibility = resistibility.edge_resistibility_index();
        properties_computed.insert(
            "edge_resistibility_index".to_string(),
            serde_json::to_value(index_of_edge_resistibility)?,
        );

        Ok(())
    }

    fn vertex_resistibility<Gr: Graph + Clone, C: Colourizer>(
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
            "vertex_resistibilities".to_string(),
            vertex_resistibilities_json,
        );

        let vertex_resistibility_index = resistibility.vertex_resistibility_index();
        properties_computed.insert(
            "vertex_resistibility_index".to_string(),
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

impl ChromaticPropsProcedureConfig {
    const PROC_TYPE: &'static str = "critic-and-stable-properties";

    pub fn from_proc_config(config: &HashMap<String, serde_json::Value>) -> Result<Self> {
        let colouriser_type = config_helper::resolve_value_or_default(
            &config,
            "colouriser-type",
            "bfs".to_string(),
            Self::PROC_TYPE,
        )?;
        let parallel =
            config_helper::resolve_value_or_default(&config, "parallel", true, Self::PROC_TYPE)?;
        let properties = config_helper::resolve_value(&config, "properties", Self::PROC_TYPE)?;
        let properties_to_compute = ChromaticPropertiesToCompute::new();
        let mut result = ChromaticPropsProcedureConfig {
            colouriser_type,
            parallel,
            properties_to_compute,
        };
        result.resolve_properties_to_compute(properties);
        Ok(result)
    }

    fn resolve_properties_to_compute(&mut self, properties: Vec<String>) {
        for property in properties.iter() {
            match property.as_str() {
                "critical" => {
                    self.properties_to_compute.critical = true;
                }
                "cocritical" => {
                    self.properties_to_compute.cocritical = true;
                }
                "vertex-subcritical" => {
                    self.properties_to_compute.vertex_subcritical = true;
                }
                "edge-subcritical" => {
                    self.properties_to_compute.edge_subcritical = true;
                }
                "acritical" => {
                    self.properties_to_compute.acritical = true;
                }
                "stable" => {
                    self.properties_to_compute.stable = true;
                }
                "costable" => {
                    self.properties_to_compute.costable = true;
                }
                "girth" => {
                    self.properties_to_compute.girth = true;
                }
                "cyclic-connectivity" => {
                    self.properties_to_compute.cyclic_connectivity = true;
                }
                "resistance" => {
                    self.properties_to_compute.resistance = true;
                }
                "edge-resistibility" => {
                    self.properties_to_compute.edge_resistibility = true;
                }
                "vertex-resistibility" => {
                    self.properties_to_compute.vertex_resistibility = true;
                }
                _ => {}
            }
        }
    }

    pub fn colouriser_type(&self) -> &String {
        &self.colouriser_type
    }

    pub fn parallel(&self) -> bool {
        self.parallel
    }
}

impl<G: Graph + Clone + 'static> ProcedureBuilder<G> for ChromaticPropsProcedureBuilder {
    fn build(&self, config: Config) -> procedure::Result<Box<dyn Procedure<G>>> {
        let proc_config = ChromaticPropsProcedureConfig::from_proc_config(&config)?;
        Ok(Box::new(ChromaticPropsProcedure {
            config: proc_config,
            _ph: marker::PhantomData,
        }))
    }
}
