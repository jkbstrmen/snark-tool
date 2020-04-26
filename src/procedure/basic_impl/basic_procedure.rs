use crate::error::Error;

use crate::graph::graph::Graph;
use crate::procedure::basic_impl::basic_config::BasicConfig;
use crate::procedure::basic_impl::basic_properties::BasicProperties;
use crate::procedure::procedure::{Config, Procedure};
use crate::service::colour::bfs::BFSColourizer;
use crate::service::colour::colouriser::Colourizer;
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
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::Write;
use std::{fs, path, result};

type Result<T> = result::Result<T, Error>;

pub struct BasicProcedure {
    proc_type: String,
    config: BasicConfig,
}

impl Procedure<BasicProperties> for BasicProcedure {
    fn new_with_config(proc_type: impl AsRef<str>, config: Config) -> Self {
        BasicProcedure {
            proc_type: String::from(proc_type.as_ref()),
            config: BasicConfig::from_config(config.clone(), proc_type.as_ref().to_string()),
        }
    }

    fn run<G>(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()>
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
            "write-with-properties" => {
                self.write_with_properties(graphs)?;
            }
            _ => {
                self.handle_unknown_type();
            }
        };
        Ok(())
    }
}

impl BasicProcedure {
    pub fn read_graph<G>(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()>
    where
        G: Debug + Graph,
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
                )))
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
                )))
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
            _ => {
                return Err(Error::ConfigError(String::from(
                    "unknown colouriser type for colour procedure",
                )))
            }
        }
        Ok(())
    }

    fn color_by_colourizer<G, C>(graphs: &mut Vec<(G, BasicProperties)>)
    where
        C: Colourizer,
        G: Graph,
    {
        let mut counter = 0;
        for graph in graphs {
            let result = C::is_colorable(&graph.0);
            graph.1.colorable = result;

            // temp
            println!("graph: {} is colorable: {}", counter, result);
            counter += 1;
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

    fn handle_unknown_type(&self) {
        println!("Unknown procedure type: {}", self.proc_type);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct GraphWithProperties {
    graph: String,
    properties: BasicProperties,
}
