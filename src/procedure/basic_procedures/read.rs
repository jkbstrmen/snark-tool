use crate::graph::graph::{Graph, GraphConstructor};
use crate::graph::undirected::UndirectedGraph;
use crate::procedure::error::Error;
use crate::procedure::helpers::config_helper;
use crate::procedure::procedure::{GraphProperties, Procedure, Result};
use crate::procedure::procedure_builder::{ConfigMap, ProcedureBuilder};
use crate::service::io::error::ReadError;
use crate::service::io::reader::Reader;
use crate::service::io::reader_ba::BaReader;
use crate::service::io::reader_g6::G6Reader;
use crate::service::io::reader_json::JsonReader;
use crate::service::io::reader_s6::S6Reader;
use std::collections::HashMap;
use std::{fs, marker, path};

// config params
const FILE_NAME: &str = "file";
const GRAPH_FORMAT: &str = "graph-format";
const NUMBER_OF_GRAPHS: &str = "number-of-graphs";

// config param properties
pub const G6_FORMAT: &str = "g6";
pub const S6_FORMAT: &str = "s6";
pub const BA_FORMAT: &str = "ba";
pub const JSON_FORMAT: &str = "json";

const DEFAULT_FILE_NAME: &str = "read-procedure-input-file.g6";

struct ReadProcedure<G: Graph> {
    config: ReadProcedureConfig,
    _ph: marker::PhantomData<G>,
}

impl<G: UndirectedGraph + GraphConstructor> Procedure<G> for ReadProcedure<G> {
    fn run(&self, graphs: &mut Vec<(G, GraphProperties)>) -> Result<()> {
        println!("running read procedure");
        self.read_graphs(graphs)
    }
}

impl<G: UndirectedGraph + GraphConstructor> ReadProcedure<G> {
    pub fn read_graphs(&self, graphs: &mut Vec<(G, GraphProperties)>) -> Result<()> {
        let file_path = self.config.file_path();
        let graphs_count = self.config.number_of_graphs();
        let file = Self::open_file_to_read(file_path)?;
        let graph_format = self.config.graph_format();

        match graph_format.as_str() {
            G6_FORMAT => {
                let reader = G6Reader::new(&file);
                Self::read_by_format(reader, graphs, graphs_count)?;
            }
            BA_FORMAT => {
                let reader = BaReader::<G>::new(&file);
                Self::read_by_format(reader, graphs, graphs_count)?;
            }
            S6_FORMAT => {
                let reader = S6Reader::<G>::new(&file);
                Self::read_by_format(reader, graphs, graphs_count)?;
            }
            JSON_FORMAT => {
                Self::read_json_format(graphs, graphs_count, &file)?;
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
        graphs: &mut Vec<(G, GraphProperties)>,
        graphs_count: Option<usize>,
    ) -> Result<()>
    where
        R: Reader<'a, G>,
    {
        let mut counter = 1;
        let mut graph_opt = reader.next();
        while graph_opt.is_some() {
            let graph = graph_opt.unwrap()?;
            let mut properties = GraphProperties::new();
            properties.insert("size".to_string(), serde_json::to_value(graph.size())?);
            graphs.push((graph, properties));
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

    fn read_json_format(
        graphs: &mut Vec<(G, GraphProperties)>,
        graphs_count: Option<usize>,
        file: &fs::File,
    ) -> Result<()> {
        let mut counter = 1;

        let mut reader = JsonReader::<G>::new(file);

        let mut graph_opt = reader.next_with_properties();
        while graph_opt.is_some() {
            let graph = graph_opt.unwrap()?;
            graphs.push(graph);
            counter += 1;

            if graphs_count.is_some() && graphs_count.unwrap() < counter {
                break;
            }

            graph_opt = reader.next_with_properties();
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
    file_path: String,
    graph_format: String,
    number_of_graphs: Option<usize>,
}

impl ReadProcedureConfig {
    pub const PROC_TYPE: &'static str = "read";

    pub fn new(file_path: String, graph_format: String, number_of_graphs: Option<usize>) -> Self {
        ReadProcedureConfig {
            file_path,
            graph_format,
            number_of_graphs,
        }
    }

    pub fn default() -> Self {
        Self {
            file_path: DEFAULT_FILE_NAME.to_string(),
            graph_format: GRAPH_FORMAT.to_string(),
            number_of_graphs: None,
        }
    }

    pub fn from_proc_config(config: &HashMap<String, serde_json::Value>) -> Result<Self> {
        let file_path = config_helper::resolve_value_or_default(
            &config,
            FILE_NAME,
            DEFAULT_FILE_NAME.to_string(),
            Self::PROC_TYPE,
        )?;
        let graph_format = config_helper::resolve_value_or_default(
            &config,
            GRAPH_FORMAT,
            G6_FORMAT.to_string(),
            Self::PROC_TYPE,
        )?;
        let number_of_graphs = config_helper::resolve_value_or_default(
            &config,
            NUMBER_OF_GRAPHS,
            None,
            Self::PROC_TYPE,
        )?;
        let result = ReadProcedureConfig {
            file_path,
            graph_format,
            number_of_graphs,
        };
        Ok(result)
    }

    pub fn file_path(&self) -> &String {
        &self.file_path
    }

    pub fn graph_format(&self) -> &String {
        &self.graph_format
    }

    pub fn number_of_graphs(&self) -> Option<usize> {
        self.number_of_graphs
    }
}

pub struct ReadProcedureBuilder {}

impl<G: UndirectedGraph + GraphConstructor + 'static> ProcedureBuilder<G> for ReadProcedureBuilder {
    fn build_from_map(&self, config: ConfigMap) -> Result<Box<dyn Procedure<G>>> {
        let proc_config = ReadProcedureConfig::from_proc_config(&config)?;
        Ok(Box::new(ReadProcedure {
            config: proc_config,
            _ph: marker::PhantomData,
        }))
    }
}

impl ReadProcedureBuilder {
    pub fn build<G: UndirectedGraph + GraphConstructor + 'static>(
        config: ReadProcedureConfig,
    ) -> Box<dyn Procedure<G>> {
        Box::new(ReadProcedure {
            config,
            _ph: marker::PhantomData,
        })
    }
}
