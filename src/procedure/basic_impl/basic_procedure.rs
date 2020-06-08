use crate::error::Error;

use crate::graph::graph::{Graph, GraphConstructor};
use crate::graph::undirected::simple_graph::SimpleGraph;
use crate::procedure::basic_impl::basic_config::BasicConfig;
use crate::procedure::procedure::{Config, Procedure};
use crate::service::chromatic_properties::critical_prop::CriticalProperties;
use crate::service::chromatic_properties::stable_and_critical_prop::StableAndCriticalProperties;
use crate::service::colour::bfs::BFSColourizer;
use crate::service::colour::colouriser::Colourizer;
use crate::service::colour::cvd_dfs::CvdDfsColourizer;
use crate::service::colour::cvd_sat::CvdSatColourizer;
use crate::service::colour::sat::SATColourizer;
use crate::service::io::error::{ReadError, WriteError};
use crate::service::io::reader::Reader;
use crate::service::io::reader_ba::BaReader;
use crate::service::io::reader_g6::G6Reader;
use crate::service::io::reader_s6::S6Reader;
use crate::service::io::writer_ba::BaWriter;
use crate::service::io::writer_g6::G6Writer;
use crate::service::io::writer_s6::S6Writer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc;
use std::{fs, marker, path, result, thread, time};

type Result<T> = result::Result<T, Error>;

pub struct BasicProcedure {
    proc_type: String,
    config: BasicConfig,
}

type BasicProperties = HashMap<String, String>;

impl Procedure<BasicProperties> for BasicProcedure {
    fn new_with_config(proc_type: impl AsRef<str>, config: Config) -> Self {
        BasicProcedure {
            proc_type: String::from(proc_type.as_ref()),
            config: BasicConfig::from_config(config.clone(), proc_type.as_ref().to_string()),
        }
    }

    fn run<G>(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()>
    where
        G: Debug + Graph + GraphConstructor,
    {
        match self.proc_type.as_str() {
            "read" => {
                self.read_graph(graphs)?;
            }
            "write" => {
                self.write_graph(graphs)?;
            }
            "colour" => {
                self.colour_graph(graphs)?;
            }
            "write-with-properties" => {
                self.write_with_properties(graphs)?;
            }
            "chromatic-properties" => {
                self.chromatic_properties(graphs)?;
            }
            _ => {
                self.handle_unknown_type();
            }
        };
        Ok(())
    }
}

// TODO - add G<V, E> to BasicProcedure struct
impl BasicProcedure {
    pub fn read_graph<G>(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()>
    where
        G: Debug + Graph + GraphConstructor,
    {
        println!("Running procedure: {}", self.proc_type);
        let file_path = self.config.get_file()?;
        let graphs_count = self.config.get_number_of_graphs()?;
        let file = BasicProcedure::open_file_to_read(file_path)?;
        let graph_format = self.config.get_graph_format()?;

        match graph_format.as_str() {
            "g6" => {
                let reader = G6Reader::<G>::new(&file);
                BasicProcedure::read_by_format(reader, graphs, graphs_count)?;
            }
            "ba" => {
                let reader = BaReader::<G>::new(&file);
                BasicProcedure::read_by_format(reader, graphs, graphs_count)?;
            }
            "s6" => {
                let reader = S6Reader::<G>::new(&file);
                BasicProcedure::read_by_format(reader, graphs, graphs_count)?;
            }
            _ => {
                return Err(Error::ConfigError(String::from(
                    "unknown graph format for read procedure",
                )));
            }
        }
        Ok(())
    }

    fn read_by_format<'a, G, R>(
        mut reader: R,
        graphs: &mut Vec<(G, BasicProperties)>,
        graphs_count: Option<usize>,
    ) -> Result<()>
    where
        R: Reader<'a, G>,
        G: Graph,
    {
        let mut counter = 1;
        let mut graph_opt = reader.next();
        while graph_opt.is_some() {
            let graph = graph_opt.unwrap()?;
            graphs.push((graph, BasicProperties::new()));
            counter += 1;

            if graphs_count.is_some() && graphs_count.unwrap() < counter {
                break;
            }

            graph_opt = reader.next();
        }
        if graphs_count.is_some() && graphs_count.unwrap() > counter {
            println!(
                "You asked for: {} graphs but given file contains only {}",
                graphs_count.unwrap(),
                counter
            );
        }
        Ok(())
    }

    pub fn write_graph<G>(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()>
    where
        G: Graph + Debug,
    {
        println!("Running procedure: {}", self.proc_type);
        let file_path = self.config.get_file()?;
        let graph_format = self.config.get_graph_format()?;

        match graph_format.as_str() {
            "g6" => {
                G6Writer::write_graphs_to_file(&graphs, file_path)?;
            }
            "ba" => {
                BaWriter::write_graphs_to_file(graphs, file_path)?;
            }
            "s6" => {
                S6Writer::write_graphs_to_file(graphs, file_path)?;
            }
            _ => {
                return Err(Error::ConfigError(String::from(
                    "unknown graph format for read procedure",
                )));
            }
        }

        Ok(())
    }

    pub fn colour_graph<G>(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()>
    where
        G: Debug + Graph,
    {
        println!("Running procedure: {}", self.proc_type);

        let colouriser_type_opt = self.config.get_colouriser_type()?;
        let colouriser_type;
        if colouriser_type_opt.is_none() {
            // resolve according to graph size

            colouriser_type = "bfs";
        } else {
            colouriser_type = colouriser_type_opt.unwrap();
        }

        match colouriser_type {
            "bfs" => {
                BasicProcedure::color_by_colourizer::<G, BFSColourizer>(graphs);
            }
            "sat" => {
                BasicProcedure::color_by_colourizer::<G, SATColourizer>(graphs);
            }
            "cvd-dfs" => {
                BasicProcedure::color_by_colourizer::<G, CvdDfsColourizer>(graphs);
            }
            _ => {
                return Err(Error::ConfigError(String::from(
                    "unknown colouriser type for colour procedure",
                )));
            }
        }
        Ok(())
    }

    fn color_by_colourizer<G, C>(graphs: &mut Vec<(G, BasicProperties)>)
    where
        C: Colourizer,
        G: Graph,
    {
        for graph in graphs {
            let result = C::is_colorable(&graph.0);
            graph
                .1
                .insert("colourable".to_string(), format!("{}", result));
        }
    }

    pub fn write_with_properties<G>(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()>
    where
        G: Graph,
    {
        let file_path = self.config.get_file()?;
        let mut file = BasicProcedure::open_file_to_write(file_path)?;
        let graph_format = self.config.get_graph_format()?;
        let mut vec = vec![];
        for graph in graphs {
            let graph_string;
            match graph_format.as_str() {
                "g6" => {
                    graph_string = G6Writer::graph_to_g6_string(&graph.0);
                }
                "s6" => {
                    graph_string = S6Writer::graph_to_s6_string(&graph.0);
                }
                _ => {
                    return Err(Error::ConfigError(format!(
                        "unknown graph format: '{}' for procedure: {}",
                        graph_format, self.proc_type
                    )));
                }
            }
            let graph_with_properties = GraphWithProperties {
                graph: graph_string,
                properties: graph.1.clone(),
            };
            vec.push(graph_with_properties);
        }
        let serialized = serde_json::to_string_pretty(&vec).unwrap();
        let result = writeln!(file, "{}", serialized);
        if let Err(err) = result {
            return Err(Error::WriteError(WriteError {
                message: format!("error while writing to file: {}, error: {}", file_path, err),
            }));
        }
        Ok(())
    }

    fn open_file_to_read<P: AsRef<path::Path>>(path: P) -> Result<fs::File> {
        let file_result = OpenOptions::new().read(true).open(&path);
        if file_result.is_err() {
            return Err(Error::ReadError(ReadError {
                message: format!("open file to read error for file: {:?}", path.as_ref()),
            }));
        }
        Ok(file_result.unwrap())
    }

    fn open_file_to_write<P: AsRef<path::Path>>(path: P) -> Result<fs::File> {
        let file_result = OpenOptions::new().write(true).create(true).open(&path);
        if file_result.is_err() {
            return Err(Error::ReadError(ReadError {
                message: format!("open file to write error for file: {:?}", path.as_ref()),
            }));
        }
        Ok(file_result.unwrap())
    }

    fn chromatic_properties<G: Graph>(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()> {
        println!("Running procedure: {}", self.proc_type);

        let parallel = self.config.get_parallel()?;
        let colourizer_type = self.config.get_colouriser_type()?;
        match colourizer_type {
            Some(col_type) => match col_type.as_str() {
                "sat" => {
                    return self.critical_and_stable_properties(
                        graphs,
                        SATColourizer::new(),
                        parallel,
                    );
                }
                "bfs" => {
                    return self.critical_and_stable_properties(
                        graphs,
                        BFSColourizer::new(),
                        parallel,
                    );
                }
                "cvd-bfs" => {
                    return self.critical_and_stable_properties(
                        graphs,
                        CvdDfsColourizer::new(),
                        parallel,
                    );
                }
                "cvd-sat" => {
                    return self.critical_and_stable_properties(
                        graphs,
                        CvdSatColourizer::new(),
                        parallel,
                    );
                }
                _ => {
                    return Err(Error::ConfigError(format!(
                        "unknown colourizer: {} for chromatic properties",
                        col_type
                    )));
                }
            },
            None => {
                return self.critical_and_stable_properties(graphs, BFSColourizer::new(), parallel);
            }
            _ => {
                // return err
            }
        }
        Ok(())
    }

    fn critical_and_stable_properties<G: Graph, C: Colourizer>(
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
    fn critical_and_stable_properties_sequential<G: Graph, C: Colourizer>(
        &self,
        graphs: &mut Vec<(G, BasicProperties)>,
        colourizer: C,
    ) -> Result<()> {
        let mut critical = 0;
        let mut cocritical = 0;
        let mut vsubcritical = 0;
        let mut esubcritical = 0;
        let mut stable = 0;
        let mut costable = 0;

        let mut index = 0;

        for graph in graphs {
            // temp
            let begin = time::Instant::now();

            let mut props =
                StableAndCriticalProperties::of_graph_with_colourizer(&graph.0, C::new());
            critical += props.is_critical() as usize;
            cocritical += props.is_cocritical() as usize;
            vsubcritical += props.is_vertex_subcritical() as usize;
            esubcritical += props.is_edge_subcritical() as usize;
            stable += props.is_stable() as usize;
            costable += props.is_costable() as usize;

            // temp
            println!(
                "graph: {} elapsed: {} ms",
                index,
                begin.elapsed().as_millis()
            );

            index += 1;
        }

        // temp
        println!("CRITICAL: {}", critical);
        println!("COCRITICAL: {}", cocritical);
        println!("VERTEX SUBCRITICAL: {}", vsubcritical);
        println!("EDGE SUBCRITICAL: {}", esubcritical);
        println!("STABLE: {}", stable);
        println!("COSTABLE: {}", costable);

        Ok(())
    }

    fn critical_and_stable_properties_in_parallel<G: Graph, C: Colourizer>(
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
            let handle = Self::thread_for_graph(graph_local, index, tx_cloned, &colourizer);
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
                let handle = Self::thread_for_graph(graph_local, index, tx_cloned, &colourizer);
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
        let mut index = 0;
        for result in results {
            graphs[index]
                .1
                .insert("critical".to_string(), format!("{}", result.critical));
            graphs[index]
                .1
                .insert("cocritical".to_string(), format!("{}", result.cocritical));
            graphs[index].1.insert(
                "vertex_subcritical".to_string(),
                format!("{}", result.vertex_subcritical),
            );
            graphs[index].1.insert(
                "edge_subcritical".to_string(),
                format!("{}", result.edge_subcritical),
            );
            graphs[index]
                .1
                .insert("stable".to_string(), format!("{}", result.stable));
            graphs[index]
                .1
                .insert("costable".to_string(), format!("{}", result.costable));
            index += 1;
        }
        for thread in threads {
            thread.join().unwrap();
        }
        Ok(())
    }

    fn thread_for_graph<C: Colourizer>(
        graph: SimpleGraph,
        index: usize,
        sender: mpsc::Sender<ChromaticPropertiesResult>,
        _colourizer: &C
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

    fn critical_properties_sequential<G: Graph>(
        &self,
        graphs: &mut Vec<(G, BasicProperties)>,
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

    fn critical_properties_in_parallel<G: Graph>(
        &self,
        graphs: &mut Vec<(G, BasicProperties)>,
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

    fn filter<G: Graph>(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()> {
        let mut filtered = vec![];
        filtered.push((graphs[0].0.clone(), graphs[0].1.clone()));
        *graphs = filtered;
        Ok(())
    }

    fn handle_unknown_type(&self) {
        println!("Unknown procedure type: {}", self.proc_type);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct GraphWithProperties {
    graph: String,
    properties: BasicProperties,
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
