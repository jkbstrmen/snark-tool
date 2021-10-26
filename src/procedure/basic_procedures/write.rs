use crate::graph::graph::GraphConstructor;
use crate::graph::undirected::UndirectedGraph;
use crate::procedure::basic_procedures::read;
use crate::procedure::error::Error;
use crate::procedure::helpers::config_helper;
use crate::procedure::procedure::{GraphProperties, Procedure, Result};
use crate::procedure::procedure_builder::{ConfigMap, ProcedureBuilder};
use crate::service::io::error::{ReadError, WriteError};
use crate::service::io::writer_ba::BaWriter;
use crate::service::io::writer_g6::G6Writer;
use crate::service::io::writer_s6::S6Writer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::{fs, marker, path};

// config params
const FILE_NAME: &str = "file";
const GRAPH_FORMAT: &str = "graph-format";
const WITH_PROPERTIES: &str = "with-properties";

const DEFAULT_FILE_NAME: &str = "write-procedure-output-file";

struct WriteProcedure<G: UndirectedGraph> {
    config: WriteProcedureConfig,
    _ph: marker::PhantomData<G>,
}

pub struct WriteProcedureBuilder {}

pub struct WriteProcedureConfig {
    file_path: String,
    graph_format: String,
    with_properties: bool,
}

impl<G: UndirectedGraph> Procedure<G> for WriteProcedure<G> {
    fn run(&self, graphs: &mut Vec<(G, GraphProperties)>) -> Result<()> {
        println!("running write procedure");
        self.write_graphs(graphs)
    }
}

impl<G: UndirectedGraph> WriteProcedure<G> {
    pub fn write_graphs(&self, graphs: &mut Vec<(G, GraphProperties)>) -> Result<()>
    where
        G: UndirectedGraph,
    {
        let file_path = self.config.file_path();
        let graph_format = self.config.graph_format();
        let with_properties = self.config.with_properties();

        if with_properties {
            return self.write_with_properties(graphs, graph_format, file_path);
        }
        self.write_without_properties(graphs, graph_format, file_path)
    }

    fn write_without_properties(
        &self,
        graphs: &mut Vec<(G, GraphProperties)>,
        graph_format: &String,
        file_path: &String,
    ) -> Result<()> {
        match graph_format.as_str() {
            read::G6_FORMAT => {
                G6Writer::write_graphs_to_file(&graphs, file_path)?;
            }
            read::BA_FORMAT => {
                BaWriter::write_graphs_to_file(graphs, file_path)?;
            }
            read::S6_FORMAT => {
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

    fn write_with_properties(
        &self,
        graphs: &mut Vec<(G, GraphProperties)>,
        graph_format: &String,
        file_path: &String,
    ) -> Result<()> {
        let mut file = Self::open_file_to_write(file_path)?;
        let mut vec = vec![];
        for graph in graphs {
            let graph_string;
            match graph_format.as_str() {
                read::G6_FORMAT => {
                    graph_string = G6Writer::graph_to_g6_string(&graph.0);
                }
                read::S6_FORMAT => {
                    graph_string = S6Writer::graph_to_s6_string(&graph.0);
                }
                _ => {
                    return Err(Error::ConfigError(format!(
                        "unknown graph format: '{}' for procedure: {}",
                        graph_format,
                        WriteProcedureConfig::PROC_TYPE
                    )));
                }
            }
            let graph_with_properties = GraphWithProperties {
                graph: graph_string,
                properties: graph.1.clone(),
                graph_format: graph_format.clone(),
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

    fn open_file_to_write<P: AsRef<path::Path>>(path: P) -> Result<fs::File> {
        let file_result = OpenOptions::new().write(true).create(true).open(&path);
        if file_result.is_err() {
            return Err(Error::ReadError(ReadError {
                message: format!("open file to write error for file: {:?}", path.as_ref()),
            }));
        }
        Ok(file_result.unwrap())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GraphWithProperties {
    pub graph: String,
    pub properties: GraphProperties,
    pub graph_format: String,
}

impl WriteProcedureConfig {
    pub const PROC_TYPE: &'static str = "write";

    pub fn new(file_path: String, graph_format: String, with_properties: bool) -> Self {
        WriteProcedureConfig {
            file_path,
            graph_format,
            with_properties,
        }
    }

    pub fn default() -> Self {
        WriteProcedureConfig {
            file_path: DEFAULT_FILE_NAME.to_string(),
            graph_format: read::G6_FORMAT.to_string(),
            with_properties: false,
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
            read::G6_FORMAT.to_string(),
            Self::PROC_TYPE,
        )?;
        let with_properties =
            config_helper::resolve_value(&config, WITH_PROPERTIES, Self::PROC_TYPE)?;

        let result = WriteProcedureConfig {
            file_path,
            graph_format,
            with_properties,
        };
        Ok(result)
    }
    pub fn file_path(&self) -> &String {
        &self.file_path
    }

    pub fn graph_format(&self) -> &String {
        &self.graph_format
    }

    pub fn with_properties(&self) -> bool {
        self.with_properties
    }
}

impl<G: UndirectedGraph + GraphConstructor + 'static> ProcedureBuilder<G>
    for WriteProcedureBuilder
{
    fn build_from_map(&self, config: ConfigMap) -> Result<Box<dyn Procedure<G>>> {
        let proc_config = WriteProcedureConfig::from_proc_config(&config)?;
        Ok(Box::new(WriteProcedure {
            config: proc_config,
            _ph: marker::PhantomData,
        }))
    }
}

impl WriteProcedureBuilder {
    pub fn build<G: UndirectedGraph + 'static>(
        config: WriteProcedureConfig,
    ) -> Box<dyn Procedure<G>> {
        Box::new(WriteProcedure {
            config,
            _ph: marker::PhantomData,
        })
    }
}
