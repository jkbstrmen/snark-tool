use crate::error::Error;
use crate::graph::graph::{Graph, GraphConstructor};
use crate::procedure::configuration::ProcedureConfig;
use crate::procedure::procedure::Config;
use crate::procedure::procedure::{BasicProperties, Procedure, Result};
use crate::procedure::procedure_builder::ProcedureBuilder;
use crate::service::io::error::ReadError;
use crate::service::io::reader::Reader;
use crate::service::io::reader_ba::BaReader;
use crate::service::io::reader_g6::G6Reader;
use crate::service::io::reader_s6::S6Reader;
use std::collections::HashMap;
use std::str::FromStr;
use std::{fmt, fs, marker, path, result};

struct ReadProcedure<G: Graph> {
    config: ReadProcedureConfig,
    _ph: marker::PhantomData<G>,
}

impl<G: Graph + GraphConstructor> Procedure<G> for ReadProcedure<G> {
    fn run(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()> {
        println!("running read procedure");
        self.read_graphs(graphs)
    }
}

impl<G: Graph + GraphConstructor> ReadProcedure<G> {
    pub fn read_graphs(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()> {
        let file_path = self.config.file_path()?;
        let graphs_count = self.config.number_of_graphs()?;
        let file = Self::open_file_to_read(file_path)?;
        let graph_format = self.config.graph_format()?;

        match graph_format.as_str() {
            "g6" => {
                let reader = G6Reader::new(&file);
                Self::read_by_format(reader, graphs, graphs_count)?;
            }
            "ba" => {
                let reader = BaReader::<G>::new(&file);
                Self::read_by_format(reader, graphs, graphs_count)?;
            }
            "s6" => {
                let reader = S6Reader::<G>::new(&file);
                Self::read_by_format(reader, graphs, graphs_count)?;
            }
            _ => {
                return Err(Error::ConfigError(String::from(
                    "unknown graph format for read procedure",
                )));
            }
        }
        Ok(())
    }

    fn read_by_format<'a, R>(
        mut reader: R,
        graphs: &mut Vec<(G, BasicProperties)>,
        graphs_count: Option<usize>,
    ) -> Result<()>
    where
        R: Reader<'a, G>,
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

    fn open_file_to_read<P: AsRef<path::Path>>(path: P) -> Result<fs::File> {
        let file_result = fs::OpenOptions::new().read(true).open(&path);
        if file_result.is_err() {
            return Err(Error::ReadError(ReadError {
                message: format!("open file to read error for file: {:?}", path.as_ref()),
            }));
        }
        Ok(file_result.unwrap())
    }
}

pub struct ReadProcedureConfig {
    config: HashMap<String, String>,
}

// TODO - move to base class
impl ReadProcedureConfig {
    const PROC_TYPE: &'static str = "read";

    pub fn from_map(config: HashMap<String, String>) -> Self {
        ReadProcedureConfig { config }
    }

    pub fn file_path(&self) -> Result<&String> {
        let file_path_opt = self.config.get("file");
        if file_path_opt.is_none() {
            return Err(Error::ConfigError(format!(
                "file not specified for procedure: {}",
                Self::PROC_TYPE
            )));
        }
        Ok(file_path_opt.unwrap())
    }

    pub fn graph_format(&self) -> Result<&String> {
        let graph_format = self.config.get("graph-format");
        if graph_format.is_none() {
            return Err(Error::ConfigError(format!(
                "missing graph format for procedure: {}",
                Self::PROC_TYPE
            )));
        }
        Ok(graph_format.unwrap())
    }

    pub fn number_of_graphs(&self) -> Result<Option<usize>> {
        let graphs_count_opt = self.config.get("number-of-graphs");
        let graphs_count;
        if graphs_count_opt.is_none() {
            graphs_count = None;
        } else {
            graphs_count = Option::Some(
                u64::from_str(graphs_count_opt.unwrap().clone().as_str()).unwrap() as usize,
            );
        }
        Ok(graphs_count)
    }
}

pub struct ReadProcedureBuilder {}

impl<G: Graph + GraphConstructor + 'static> ProcedureBuilder<G> for ReadProcedureBuilder {
    fn build(&self, config: Config) -> Box<dyn Procedure<G>> {
        Box::new(ReadProcedure {
            config: ReadProcedureConfig::from_map(config),
            _ph: marker::PhantomData,
        })
    }
}
