use crate::error::Error;
use crate::graph::graph::{Graph, GraphConstructor};
use crate::procedure::config_helper;
use crate::procedure::procedure::{GraphProperties, Procedure, Result};
use crate::procedure::procedure_builder::{Config, ProcedureBuilder};
use crate::service::io::error::ReadError;
use crate::service::io::reader::Reader;
use crate::service::io::reader_ba::BaReader;
use crate::service::io::reader_g6::G6Reader;
use crate::service::io::reader_json::JsonReader;
use crate::service::io::reader_s6::S6Reader;
use std::collections::HashMap;
use std::{fs, marker, path};

struct ReadProcedure<G: Graph> {
    config: ReadProcedureConfig,
    _ph: marker::PhantomData<G>,
}

impl<G: Graph + GraphConstructor> Procedure<G> for ReadProcedure<G> {
    fn run(&self, graphs: &mut Vec<(G, GraphProperties)>) -> Result<()> {
        println!("running read procedure");
        self.read_graphs(graphs)
    }
}

impl<G: Graph + GraphConstructor> ReadProcedure<G> {
    pub fn read_graphs(&self, graphs: &mut Vec<(G, GraphProperties)>) -> Result<()> {
        let file_path = self.config.file_path();
        let graphs_count = self.config.number_of_graphs();
        let file = Self::open_file_to_read(file_path)?;
        let graph_format = self.config.graph_format();

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
            "json" => {
                Self::read_json_format(graphs, graphs_count, &file);
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
            graphs.push((graph, GraphProperties::new()));
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
    const PROC_TYPE: &'static str = "read";

    pub fn from_proc_config(config: &HashMap<String, serde_json::Value>) -> Result<Self> {
        let file_path = config_helper::resolve_value_or_default(
            &config,
            "file",
            "write-procedure-output-file".to_string(),
            Self::PROC_TYPE,
        )?;
        let graph_format = config_helper::resolve_value_or_default(
            &config,
            "graph-format",
            "g6".to_string(),
            Self::PROC_TYPE,
        )?;
        let number_of_graphs = config_helper::resolve_value_or_default(
            &config,
            "number-of-graphs",
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

impl<G: Graph + GraphConstructor + 'static> ProcedureBuilder<G> for ReadProcedureBuilder {
    fn build(&self, config: Config) -> Result<Box<dyn Procedure<G>>> {
        let proc_config = ReadProcedureConfig::from_proc_config(&config)?;
        Ok(Box::new(ReadProcedure {
            config: proc_config,
            _ph: marker::PhantomData,
        }))
    }
}
