use crate::error::Error;

use crate::graph::traits::graph::Graph;
use crate::service::colour::bfs::BFSColorizer;
use crate::service::io::error::ReadError;
use crate::service::io::reader::Reader;
use crate::service::io::reader_ba::BaReader;
use crate::service::io::reader_g6::G6Reader;
use crate::service::io::reader_s6::S6Reader;
use crate::service::io::writer_ba::BaWriter;
use crate::service::io::writer_g6::G6Writer;
use crate::service::io::writer_s6::S6Writer;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::OpenOptions;
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
    // pub fn new(proc_type: impl AsRef<str>) -> Self {
    //     BasicProcedure {
    //         proc_type: String::from(proc_type.as_ref()),
    //         config: HashMap::default(),
    //     }
    // }

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
    pub fn read_graph<G>(&self, graphs: &mut Vec<G>) -> Result<()>
    where
        G: Debug + Graph,
    {
        println!("Running procedure: {}", self.proc_type);
        let file_path_opt = self.config.get("file");
        if file_path_opt.is_none() {
            return Err(Error::ConfigError(String::from("input file not specified")));
        }
        let file_path = file_path_opt.unwrap();

        let graphs_count = self.get_graphs_count();

        let file_result = OpenOptions::new().read(true).open(file_path);
        if file_result.is_err() {
            return Err(Error::ReadError(ReadError {
                message: String::from("input file read error"),
            }));
        }
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

    // move to basic config??
    fn get_graphs_count(&self) -> Option<usize>{
        let graphs_count_opt = self.config.get("number-of-graphs");
        let graphs_count;
        if graphs_count_opt.is_none() {
            graphs_count = None;
        } else {
            graphs_count = Option::Some(
                u64::from_str(graphs_count_opt.unwrap().clone().as_str()).unwrap() as usize,
            );
        }
        return graphs_count;
    }

    fn read_by_format<'a, G, R>(
        mut reader: R,
        graphs: &mut Vec<G>,
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
            graphs.push(graph);
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

        // for _i in 0..graphs_count {
        //     let graph = reader.next();
        //
        //     if graph.is_some() {
        //         let graph = graph.unwrap()?;
        //         graphs.push(graph);
        //     } else {
        //         println!(
        //             "You asked for: {} graphs but given file contains only {}",
        //             graphs_count, _i
        //         );
        //         break;
        //     }
        // }
    }

    pub fn write_graph<G>(&self, graphs: &mut Vec<G>) -> Result<()>
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

    pub fn colour_graph<G>(&self, graphs: &mut Vec<G>) -> Result<()>
    where
        G: Debug + Graph,
    {
        println!(
            "Running procedure: {} on graphs: {:?}",
            self.proc_type, graphs
        );

        let mut counter = 0;
        for graph in graphs {
            let result = BFSColorizer::is_colorable(graph);
            let result = if result { "true" } else { "false" };
            println!("graph: {} is colorable: {}", counter, result);
            counter += 1;
        }

        Ok(())
    }

    fn handle_unknown_type(&self) {
        println!("Unknown procedure type: {}", self.proc_type);
    }
}
