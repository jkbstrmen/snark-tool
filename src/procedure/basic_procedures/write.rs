use crate::error::Error;
use crate::graph::graph::{Graph, GraphConstructor};
use crate::procedure::procedure::Config;
use crate::procedure::procedure::{BasicProperties, Procedure};
use crate::procedure::procedure_builder::ProcedureBuilder;
use crate::service::io::writer_ba::BaWriter;
use crate::service::io::writer_g6::G6Writer;
use crate::service::io::writer_s6::S6Writer;
use std::collections::HashMap;
use std::{marker, result};

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
}

impl<G: Graph + GraphConstructor + 'static> ProcedureBuilder<G> for WriteProcedureBuilder {
    fn build(&self, config: Config) -> Box<dyn Procedure<G>> {
        Box::new(WriteProcedure {
            config: WriteProcedureConfig::from_map(config),
            _ph: marker::PhantomData,
        })
    }
}
