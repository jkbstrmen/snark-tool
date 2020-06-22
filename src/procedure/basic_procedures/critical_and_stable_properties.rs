use crate::error::Error;
use crate::graph::graph::Graph;
use crate::graph::undirected::simple_graph::SimpleGraph;
use crate::procedure::procedure::{BasicProperties, Config, Procedure, Result};
use crate::procedure::procedure_builder::ProcedureBuilder;
use crate::service::chromatic_properties::critical_prop::CriticalProperties;
use crate::service::chromatic_properties::stable_and_critical_prop::StableAndCriticalProperties;
use crate::service::colour::bfs::BFSColourizer;
use crate::service::colour::colouriser::Colourizer;
use crate::service::colour::cvd_dfs::CvdDfsColourizer;
use crate::service::colour::sat::SATColourizer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::mpsc;
use std::{marker, thread};

struct CriticAndStablePropsProcedure<G> {
    config: CriticAndStablePropsProcedureConfig,
    _ph: marker::PhantomData<G>,
}

struct CriticAndStablePropsProcedureConfig {
    config: HashMap<String, String>,
}

pub struct CriticAndStablePropsProcedureBuilder {}

impl<G: Graph + Clone> Procedure<G> for CriticAndStablePropsProcedure<G> {
    fn run(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()> {
        println!("running critical and stable properties procedure");
        self.critical_and_stable_properties(graphs)
    }
}

impl<G: Graph + Clone> CriticAndStablePropsProcedure<G> {
    fn critical_and_stable_properties(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()> {
        let parallel = self.config.parallel()?;
        let colourizer_type = self.config.colouriser_type()?;
        match colourizer_type {
            Some(col_type) => match col_type.as_str() {
                "sat" => {
                    return self.compute(graphs, SATColourizer::new(), parallel);
                }
                "bfs" => {
                    return self.compute(graphs, BFSColourizer::new(), parallel);
                }
                _ => {
                    return Err(Error::ConfigError(format!(
                        "unknown colourizer: {} for chromatic properties",
                        col_type
                    )));
                }
            },
            None => {
                return self.compute(graphs, BFSColourizer::new(), parallel);
            }
            _ => {
                // return err
            }
        }
        Ok(())
    }

    fn compute<C: Colourizer>(
        &self,
        graphs: &mut Vec<(G, BasicProperties)>,
        colourizer: C,
        parallel: bool,
    ) -> Result<()> {
        if parallel {
            // self.critical_properties_in_parallel(graphs);
            self.critical_and_stable_properties_in_parallel(graphs, colourizer);
        } else {
            // self.critical_properties_sequential(graphs);
            self.critical_and_stable_properties_sequential(graphs, colourizer);
        }
        Ok(())
    }

    // todo - refactor
    fn critical_and_stable_properties_sequential<C: Colourizer>(
        &self,
        graphs: &mut Vec<(G, BasicProperties)>,
        colourizer: C,
    ) -> Result<()> {
        let mut index = 0;

        for graph in graphs {
            let mut props =
                StableAndCriticalProperties::of_graph_with_colourizer(&graph.0, C::new());
            graph
                .1
                .insert("critical".to_string(), format!("{}", props.is_critical()));
            graph.1.insert(
                "cocritical".to_string(),
                format!("{}", props.is_cocritical()),
            );
            graph.1.insert(
                "vertex_subcritical".to_string(),
                format!("{}", props.is_vertex_subcritical()),
            );
            graph.1.insert(
                "edge_subcritical".to_string(),
                format!("{}", props.is_edge_subcritical()),
            );
            graph
                .1
                .insert("stable".to_string(), format!("{}", props.is_stable()));
            graph
                .1
                .insert("costable".to_string(), format!("{}", props.is_costable()));

            index += 1;
        }
        Ok(())
    }

    fn critical_and_stable_properties_in_parallel<C: Colourizer>(
        &self,
        graphs: &mut Vec<(G, BasicProperties)>,
        colourizer: C,
    ) -> Result<()> {
        let mut threads = vec![];
        let mut index = 0;
        let (tx, rx) = mpsc::channel();
        let cpus_count = num_cpus::get();

        // init first threads
        let mut graphs_iter = graphs.iter();
        let mut next_graph = graphs_iter.next();
        while next_graph.is_some() {
            let graph = next_graph.unwrap();
            let graph_local = SimpleGraph::from_graph(&graph.0);
            let tx_cloned = mpsc::Sender::clone(&tx);
            let handle = Self::spawn_thread_for_graph(graph_local, index, tx_cloned, &colourizer);
            threads.push(handle);
            index += 1;

            if index >= cpus_count {
                break;
            }
            next_graph = graphs_iter.next();
        }

        let mut results = Vec::with_capacity(graphs.len());
        // receive results and create new threads while next graphs exists
        for received in &rx {
            results.push(received);

            next_graph = graphs_iter.next();
            if next_graph.is_some() {
                let graph = next_graph.unwrap();
                let graph_local = SimpleGraph::from_graph(&graph.0);
                let tx_cloned = mpsc::Sender::clone(&tx);
                let handle =
                    Self::spawn_thread_for_graph(graph_local, index, tx_cloned, &colourizer);
                threads.push(handle);
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
            graphs[result.graph_index]
                .1
                .insert("critical".to_string(), format!("{}", result.critical));
            graphs[result.graph_index]
                .1
                .insert("cocritical".to_string(), format!("{}", result.cocritical));
            graphs[result.graph_index].1.insert(
                "vertex_subcritical".to_string(),
                format!("{}", result.vertex_subcritical),
            );
            graphs[result.graph_index].1.insert(
                "edge_subcritical".to_string(),
                format!("{}", result.edge_subcritical),
            );
            graphs[result.graph_index]
                .1
                .insert("stable".to_string(), format!("{}", result.stable));
            graphs[result.graph_index]
                .1
                .insert("costable".to_string(), format!("{}", result.costable));
        }
        for thread in threads {
            thread.join().unwrap();
        }
        Ok(())
    }

    fn spawn_thread_for_graph<C: Colourizer>(
        graph: SimpleGraph,
        index: usize,
        sender: mpsc::Sender<ChromaticPropertiesResult>,
        _colourizer: &C,
    ) -> thread::JoinHandle<()> {
        let handle = thread::spawn(move || {
            let mut props = StableAndCriticalProperties::of_graph_with_colourizer(&graph, C::new());

            let result = ChromaticPropertiesResult {
                graph_index: index,
                critical: props.is_critical(),
                cocritical: props.is_cocritical(),
                vertex_subcritical: props.is_vertex_subcritical(),
                edge_subcritical: props.is_edge_subcritical(),
                stable: props.is_stable(),
                costable: props.is_costable(),
            };
            sender.send(result);
        });
        handle
    }

    // todo - refactor
    fn critical_properties_sequential<C: Colourizer>(
        &self,
        graphs: &mut Vec<(G, BasicProperties)>,
        colourizer: C,
    ) -> Result<()> {
        let mut critical = 0;
        let mut cocritical = 0;
        let mut vsubcritical = 0;
        let mut esubcritical = 0;

        for graph in graphs {
            let mut props = CriticalProperties::of_graph(&graph.0);
            critical += props.is_critical() as usize;
            cocritical += props.is_cocritical() as usize;
            vsubcritical += props.is_vertex_subcritical() as usize;
            esubcritical += props.is_edge_subcritical() as usize;
        }

        // temp
        println!("CRITICAL: {}", critical);
        println!("COCRITICAL: {}", cocritical);
        println!("VERTEX SUBCRITICAL: {}", vsubcritical);
        println!("EDGE SUBCRITICAL: {}", esubcritical);

        Ok(())
    }

    // todo - refactor
    fn critical_properties_in_parallel<C: Colourizer>(
        &self,
        graphs: &mut Vec<(G, BasicProperties)>,
        colourizer: C,
    ) -> Result<()> {
        let mut threads = vec![];
        let mut index = 0;
        let (tx, rx) = mpsc::channel();

        for graph in graphs {
            let graph_local = SimpleGraph::from_graph(&graph.0);
            let tx_cloned = mpsc::Sender::clone(&tx);

            let handle = thread::spawn(move || {
                let mut props = CriticalProperties::of_graph(&graph_local);
                let result = ChromaticPropertiesResult {
                    graph_index: index,
                    critical: props.is_critical(),
                    cocritical: props.is_cocritical(),
                    vertex_subcritical: props.is_vertex_subcritical(),
                    edge_subcritical: props.is_edge_subcritical(),
                    stable: false,
                    costable: false,
                };
                tx_cloned.send(result);
            });
            threads.push(handle);
            index += 1;
        }

        drop(tx);
        let mut critical = 0;
        let mut cocritical = 0;
        let mut vsubcritical = 0;
        let mut esubcritical = 0;

        for received in rx {
            critical += received.critical as usize;
            cocritical += received.cocritical as usize;
            vsubcritical += received.vertex_subcritical as usize;
            esubcritical += received.edge_subcritical as usize;
        }

        // temp
        println!("===========================================");
        println!("CRITICAL: {}", critical);
        println!("COCRITICAL: {}", cocritical);
        println!("VERTEX SUBCRITICAL: {}", vsubcritical);
        println!("EDGE SUBCRITICAL: {}", esubcritical);

        for thread in threads {
            thread.join().unwrap();
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ChromaticPropertiesResult {
    graph_index: usize,
    critical: bool,
    cocritical: bool,
    vertex_subcritical: bool,
    edge_subcritical: bool,
    stable: bool,
    costable: bool,
}

impl CriticAndStablePropsProcedureConfig {
    const PROC_TYPE: &'static str = "read";

    pub fn from_map(config: HashMap<String, String>) -> Self {
        CriticAndStablePropsProcedureConfig { config }
    }

    pub fn parallel(&self) -> Result<bool> {
        let parallel_opt = self.config.get("parallel");
        let mut parallel = true;
        if parallel_opt.is_some() {
            if parallel_opt.unwrap() == "true" {
                parallel = true;
            }
            if parallel_opt.unwrap() == "false" {
                parallel = false;
            }
        }
        Ok(parallel)
    }

    pub fn colouriser_type(&self) -> Result<Option<&String>> {
        let colouriser_type_opt = self.config.get("colouriser-type");
        let colouriser_type;
        if colouriser_type_opt.is_none() {
            colouriser_type = None;
        } else {
            colouriser_type = Option::Some(colouriser_type_opt.unwrap());
        }
        Ok(colouriser_type)
    }
}

impl<G: Graph + Clone + 'static> ProcedureBuilder<G> for CriticAndStablePropsProcedureBuilder {
    fn build(&self, config: Config) -> Box<dyn Procedure<G>> {
        Box::new(CriticAndStablePropsProcedure {
            config: CriticAndStablePropsProcedureConfig::from_map(config),
            _ph: marker::PhantomData,
        })
    }
}
