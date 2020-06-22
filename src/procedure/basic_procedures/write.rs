use crate::error::Error;
use crate::graph::graph::{Graph, GraphConstructor};
use crate::procedure::procedure::Config;
use crate::procedure::procedure::{BasicProperties, Procedure};
use crate::procedure::procedure_builder::ProcedureBuilder;
use crate::service::io::error::{ReadError, WriteError};
use crate::service::io::writer_ba::BaWriter;
use crate::service::io::writer_g6::G6Writer;
use crate::service::io::writer_s6::S6Writer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::{fs, marker, path, result};

pub type Result<T> = result::Result<T, Error>;

struct WriteProcedure<G: Graph> {
    config: WriteProcedureConfig,
    _ph: marker::PhantomData<G>,
}

pub struct WriteProcedureBuilder {}

pub struct WriteProcedureConfig {
    config: HashMap<String, String>,
}

impl<G: Graph + GraphConstructor> Procedure<G> for WriteProcedure<G> {
    fn run(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()> {
        println!("running write procedure");
        self.write_graphs(graphs)
    }
}

impl<G: Graph + GraphConstructor> WriteProcedure<G> {
    pub fn write_graphs(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()>
    where
        G: Graph,
    {
        let file_path = self.config.file_path()?;
        let graph_format = self.config.graph_format()?;
        let with_properties = self.config.with_properties()?;

        if with_properties {
            return self.write_with_properties(graphs, graph_format, file_path);
        }
        self.write_without_properties(graphs, graph_format, file_path)
    }

    fn write_without_properties(
        &self,
        graphs: &mut Vec<(G, BasicProperties)>,
        graph_format: &String,
        file_path: &String,
    ) -> Result<()> {
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

    fn write_with_properties(
        &self,
        graphs: &mut Vec<(G, BasicProperties)>,
        graph_format: &String,
        file_path: &String,
    ) -> Result<()> {
        let mut file = Self::open_file_to_write(file_path)?;
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
                        graph_format,
                        WriteProcedureConfig::PROC_TYPE
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
struct GraphWithProperties {
    graph: String,
    properties: BasicProperties,
}

impl WriteProcedureConfig {
    const PROC_TYPE: &'static str = "write";

    pub fn from_map(config: HashMap<String, String>) -> Self {
        WriteProcedureConfig { config }
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

    pub fn with_properties(&self) -> Result<bool> {
        let with_opt = self.config.get("with-properties");
        let mut with_properties = true;
        if with_opt.is_some() {
            if with_opt.unwrap() == "false" {
                with_properties = false;
            }
        }
        Ok(with_properties)
    }
}

impl<G: Graph + GraphConstructor + 'static> ProcedureBuilder<G> for WriteProcedureBuilder {
    fn build(&self, config: Config) -> Box<dyn Procedure<G>> {
        Box::new(WriteProcedure {
            config: WriteProcedureConfig::from_map(config),
            _ph: marker::PhantomData,
        })
    }
}
