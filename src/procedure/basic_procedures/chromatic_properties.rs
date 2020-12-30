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
use crate::service::colour::colouriser::Colouriser;
use crate::service::colour::dfs_improved::DFSColourizer;
use crate::service::colour::sat::SATColourizer;
use crate::service::property::oddness::Oddness;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::mpsc;
use std::{marker, result, thread};

pub type Result<T> = result::Result<T, ChromaticPropertiesError>;
const DFS_COLOURISER: &str = "dfs";
const SAT_COLOURISER: &str = "sat";

// property
const CRITICAL: &str = "critical";
const COCRITICAL: &str = "cocritical";
const VERTEX_SUBCRITICAL: &str = "vertex-subcritical";
const EDGE_SUBCRITICAL: &str = "edge-subcritical";
const ACRITICAL: &str = "acritical";
const STABLE: &str = "stable";
const COSTABLE: &str = "costable";
const RESISTANCE: &str = "resistance";
const GIRTH: &str = "girth";
const CYCLIC_CONNECTIVITY: &str = "cyclic-connectivity";
const EDGE_RESISTIBILITY: &str = "edge-resistibility";
const VERTEX_RESISTIBILITY: &str = "vertex-resistibility";
const ODDNESS: &str = "oddness";

const VERTEX_RESISTIBILITIES: &str = "vertex-resistibilities";
const VERTEX_RESISTIBILITY_INDEX: &str = "vertex-resistibility-index";
const EDGE_RESISTIBILITIES: &str = "edge-resistibilities";
const EDGE_RESISTIBILITY_INDEX: &str = "edge-resistibility-index";

// property name
const COLOURISER_TYPE: &str = "colouriser-type";
const PARALLEL: &str = "parallel";
const PROPERTIES: &str = "properties";

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
    oddness: bool,
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
            oddness: false,
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
            // if graph is bigger could cause performance issues
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
            SAT_COLOURISER => {
                return Self::compute_properties(
                    graph,
                    SATColourizer::new(),
                    graph_index,
                    &properties_to_compute,
                );
            }
            DFS_COLOURISER => {
                return Self::compute_properties(
                    graph,
                    DFSColourizer::new(),
                    graph_index,
                    &properties_to_compute,
                );
            }
            _ => {
                return Err(ChromaticPropertiesError {
                    message: format!(
                        "unknown colourizer: {} to compute chromatic properties, did you mean {} or {}?",
                        colouriser_type, DFS_COLOURISER, SAT_COLOURISER
                    ),
                });
            }
        }
    }

    fn compute_properties<Gr: Graph + Clone, C: Colouriser>(
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
        if to_compute.oddness {
            // compute cyclic connectivity and add result to properties
            let oddness = Oddness::of_graph(graph);
            properties.insert(ODDNESS.to_string(), serde_json::to_value(oddness)?);
        }

        Ok(properties)
    }

    fn critical_and_stable_properties<Gr: Graph + Clone, C: Colouriser>(
        graph: &Gr,
        _colouriser: &C,
        properties_to_compute: &ChromaticPropertiesToCompute,
        properties_computed: &mut GraphProperties,
    ) -> Result<()> {
        let mut props = StableAndCriticalProperties::of_graph_with_colourizer(graph, C::new());
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

    fn critical_properties<Gr: Graph + Clone, C: Colouriser>(
        graph: &Gr,
        _colouriser: &C,
        properties_to_compute: &ChromaticPropertiesToCompute,
        properties_computed: &mut GraphProperties,
    ) -> Result<()> {
        let mut props = CriticalProperties::of_graph_with_colourizer(graph, C::new());
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
        Ok(())
    }

    fn resistance<Gr: Graph + Clone, C: Colouriser>(
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

    fn edge_resistibility<Gr: Graph + Clone, C: Colouriser>(
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

    fn vertex_resistibility<Gr: Graph + Clone, C: Colouriser>(
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

impl ChromaticPropsProcedureConfig {
    const PROC_TYPE: &'static str = "critic-and-stable-properties";

    pub fn from_proc_config(config: &HashMap<String, serde_json::Value>) -> Result<Self> {
        let colouriser_type = config_helper::resolve_value_or_default(
            &config,
            COLOURISER_TYPE,
            DFS_COLOURISER.to_string(),
            Self::PROC_TYPE,
        )?;
        let parallel =
            config_helper::resolve_value_or_default(&config, PARALLEL, true, Self::PROC_TYPE)?;
        let properties = config_helper::resolve_value(&config, PROPERTIES, Self::PROC_TYPE)?;
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
                CRITICAL => {
                    self.properties_to_compute.critical = true;
                }
                COCRITICAL => {
                    self.properties_to_compute.cocritical = true;
                }
                VERTEX_SUBCRITICAL => {
                    self.properties_to_compute.vertex_subcritical = true;
                }
                EDGE_SUBCRITICAL => {
                    self.properties_to_compute.edge_subcritical = true;
                }
                ACRITICAL => {
                    self.properties_to_compute.acritical = true;
                }
                STABLE => {
                    self.properties_to_compute.stable = true;
                }
                COSTABLE => {
                    self.properties_to_compute.costable = true;
                }
                GIRTH => {
                    self.properties_to_compute.girth = true;
                }
                CYCLIC_CONNECTIVITY => {
                    self.properties_to_compute.cyclic_connectivity = true;
                }
                RESISTANCE => {
                    self.properties_to_compute.resistance = true;
                }
                EDGE_RESISTIBILITY => {
                    self.properties_to_compute.edge_resistibility = true;
                }
                VERTEX_RESISTIBILITY => {
                    self.properties_to_compute.vertex_resistibility = true;
                }
                ODDNESS => {
                    self.properties_to_compute.oddness = true;
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
