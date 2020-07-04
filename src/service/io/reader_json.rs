use crate::graph::graph::{Graph, GraphConstructor};
use crate::procedure::basic_procedures::write::GraphWithProperties;
use crate::procedure::procedure::GraphProperties;
use crate::service::io::error::ReadError;
use crate::service::io::reader::Reader;
use crate::service::io::reader_g6::G6Reader;
use crate::service::io::reader_s6::S6Reader;
use std::fs::File;
use std::io::Read;
use std::{fs, marker, result};

type Result<T> = result::Result<T, ReadError>;

pub struct JsonReader<'a, G> {
    file: &'a fs::File,
    parsed: bool,
    position: usize,
    graphs: Vec<GraphWithProperties>,
    _ph: marker::PhantomData<G>,
}

impl<'a, G> Reader<'a, G> for JsonReader<'a, G>
where
    G: Graph + GraphConstructor,
{
    fn new(file: &'a File) -> Self {
        JsonReader {
            file,
            parsed: false,
            position: 0,
            graphs: vec![],
            _ph: marker::PhantomData,
        }
    }

    fn next(&mut self) -> Option<Result<G>> {
        if !self.parsed {
            self.parse_file();
        }
        if self.position < self.graphs.len() {
            self.position += 1;

            unimplemented!();
            // read graph by format at position
            // return graph
        }
        None
    }
}

impl<'a, G: Graph + GraphConstructor> JsonReader<'a, G> {
    fn parse_file(&mut self) -> Result<()> {
        let file_string = &mut "".to_string();
        self.file.read_to_string(file_string)?;
        let graphs_res = serde_json::from_str(file_string);
        self.graphs = graphs_res.unwrap();
        self.parsed = true;
        Ok(())
    }

    pub fn next_with_properties(&mut self) -> Option<Result<(G, GraphProperties)>> {
        if !self.parsed {
            self.parse_file();
        }
        if self.position < self.graphs.len() {
            let graph_with_properties = &self.graphs[self.position];
            self.position += 1;
            let graph = Self::read_graph(
                &graph_with_properties.graph,
                &graph_with_properties.graph_format,
            );
            if graph.is_err() {
                return Some(Err(graph.err().unwrap()));
            }
            let result = (graph.unwrap(), graph_with_properties.properties.clone());
            return Some(Ok(result));
        }
        None
    }

    fn read_graph(graph: &String, graph_format: &String) -> Result<G> {
        match graph_format.as_str() {
            "g6" => {
                return G6Reader::read_graph(graph);
            }
            "s6" => {
                return S6Reader::read_graph(graph);
            }
            _ => {
                return Err(ReadError {
                    message: format!(
                        "unknown graph format to read from json object: {}",
                        graph_format
                    ),
                });
            }
        }
    }
}
