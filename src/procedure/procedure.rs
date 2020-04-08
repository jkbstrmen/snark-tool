use crate::error::Error;

use crate::graph::traits::graph::Graph;
use crate::graph::undirected::simple_graph::SimpleGraph;
use crate::service::io::reader::Reader;
use crate::service::io::reader_ba::BaReader;
use crate::service::io::reader_g6::G6Reader;
use crate::service::io::writer_ba::BaWriter;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::path::Path;
use std::result;
use std::str::FromStr;

type Config = HashMap<String, String>;
type Result<T> = result::Result<T, Error>;

pub trait Procedure {
    fn run<G>(&self, graphs: &mut Vec<G>) -> Result<()>
    where
        G: Debug + Graph;
}

pub struct BasicProcedure {
    proc_type: String,
    config: Config,
}

impl BasicProcedure {
    pub fn new(proc_type: impl AsRef<str>) -> Self {
        BasicProcedure {
            proc_type: String::from(proc_type.as_ref()),
            config: HashMap::default(),
        }
    }

    pub fn new_with_config(proc_type: impl AsRef<str>, config: Config) -> Self {
        BasicProcedure {
            proc_type: String::from(proc_type.as_ref()),
            config,
        }
    }
}

impl Procedure for BasicProcedure {
    fn run<G>(&self, graphs: &mut Vec<G>) -> Result<()>
    where
        G: Debug + Graph,
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
            _ => {
                self.handle_unknown_type();
            }
        };
        Ok(())
    }
}

impl BasicProcedure {
    fn read_graph<G>(&self, graphs: &mut Vec<G>) -> Result<()>
    where
        G: Debug + Graph,
    {
        println!(
            "Running procedure: {} on graph: {:?}",
            self.proc_type, graphs
        );

        // handle Err
        let file_path = self.config.get("file").expect("file path not specified");

        let graphs_count_opt = self.config.get("number-of-graphs");

        // if not specified - read all graphs from file
        // handle unwrap
        let graphs_count = u64::from_str(graphs_count_opt.unwrap().clone().as_str()).unwrap();

        let file_result = OpenOptions::new().read(true).open(file_path);

        // handle unwrap
        let file = file_result.unwrap();

        let graph_format = self.config.get("graph-format");
        if graph_format.is_none() {
            return Err(Error::ConfigError(String::from(
                "missing graph format for read procedure",
            )));
        }
        let graph_format = graph_format.unwrap();

        match graph_format.as_str() {
            "g6" => {
                let mut reader = G6Reader::<G>::new(&file);
                BasicProcedure::read_by_format(reader, graphs, graphs_count as usize)?;
            }
            "ba" => {
                let mut reader = BaReader::<G>::new(&file);
                BasicProcedure::read_by_format(reader, graphs, graphs_count as usize)?;
            }
            "s6" => {}
            _ => {
                return Err(Error::ConfigError(String::from(
                    "unknown graph format for read procedure",
                )))
            }
        }
        Ok(())
    }

    fn read_by_format<'a, G, R>(
        mut reader: R,
        graphs: &mut Vec<G>,
        graphs_count: usize,
    ) -> Result<()>
    where
        R: Reader<'a, G>,
        G: Graph,
    {
        for _i in 0..graphs_count {
            let graph = reader.next();

            if graph.is_some() {
                let graph = graph.unwrap()?;
                graphs.push(graph);
            } else {
                println!(
                    "You asked for: {} graphs but given file contains only {}",
                    graphs_count, _i
                );
                break;
            }
        }
        Ok(())
    }

    fn write_graph<G>(&self, graphs: &mut Vec<G>) -> Result<()>
    where
        G: Graph + Debug,
    {
        println!(
            "Running procedure: {} on graphs: {:?}",
            self.proc_type, graphs
        );

        let file_path = self.config.get("file");
        if file_path.is_none() {
            return Err(Error::ConfigError(String::from(
                "missing file path for write procedure",
            )));
        }
        let file_path = file_path.unwrap();

        // to fn - almost same for read proc
        let graph_format = self.config.get("graph-format");
        if graph_format.is_none() {
            return Err(Error::ConfigError(String::from(
                "missing graph format for read procedure",
            )));
        }
        let graph_format = graph_format.unwrap();

        match graph_format.as_str() {
            "g6" => {
                // let mut reader = G6Reader::<G>::new(&file);
                // BasicProcedure::read_by_format(reader, graphs, graphs_count as usize);
                // G6Wri
            }
            "ba" => {
                BaWriter::write_graphs_to_file(graphs, file_path)?;
            }
            "s6" => {}
            _ => {
                return Err(Error::ConfigError(String::from(
                    "unknown graph format for read procedure",
                )))
            }
        }

        Ok(())
    }

    fn colour_graph<G>(&self, graphs: &mut Vec<G>) -> Result<()>
    where
        G: Debug + Graph,
    {
        println!(
            "Running procedure: {} on graphs: {:?}",
            self.proc_type, graphs
        );
        Ok(())
    }

    fn handle_unknown_type(&self) {
        println!("Unknown procedure type: {}", self.proc_type);
    }
}
