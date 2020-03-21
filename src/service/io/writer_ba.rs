use std::fs::{File, OpenOptions};
use std::io::{LineWriter, Read, Write};

use std::path::Path;

use petgraph::Undirected;

use petgraph::stable_graph::StableGraph;
use petgraph::visit::EdgeRef;

use crate::graph::graph::{Edge, Graph, Vertex};
use crate::service::io::error::WriteError;
use crate::service::io::reader_ba::get_graphs_count_with_preface;
use std::{io, marker, result};

type Result<T> = result::Result<T, WriteError>;

pub struct BaWriter<'a, G>
where
    G: Graph,
{
    // path: &'a Path,
    path: &'a String,
    _ph: marker::PhantomData<G>,
}

impl<'a, G> BaWriter<'a, G>
where
    G: Graph,
{
    pub fn new(path: &'a String) -> Self {
        BaWriter {
            path,
            _ph: marker::PhantomData,
        }
    }

    pub fn write_graph_ba(graph: &G, index: u32, mut buffer: impl Write) -> Result<()> {
        writeln!(buffer)?;
        writeln!(buffer, "{}", index)?;
        writeln!(buffer, "{}", graph.size())?;

        for vertex in graph.vertices() {
            for edges_of_vertex in graph.edges_of_vertex(vertex.index()) {
                if edges_of_vertex.from() == vertex.index() {
                    write!(buffer, "{} ", edges_of_vertex.to())?;
                } else {
                    write!(buffer, "{} ", edges_of_vertex.from())?;
                }
            }
            writeln!(buffer)?;
        }
        Ok(())
    }

    pub fn write_graphs_to_file(graphs: &Vec<G>, path: impl AsRef<Path>) -> Result<()> {
        let file_result = OpenOptions::new().read(true).open(&path);
        if let Err(err) = &file_result {
            if err.kind() == io::ErrorKind::NotFound {
                return BaWriter::write_graphs_to_new_file(graphs, path);
            }
        }

        let mut file = file_result?;
        return BaWriter::append_graphs_to_file(graphs, path);
    }

    fn write_graphs_to_new_file(graphs: &Vec<G>, path: impl AsRef<Path>) -> Result<()> {
        let mut file = OpenOptions::new().create(true).write(true).open(&path)?;
        writeln!(file, "{}", graphs.len())?;
        let mut index = 0;
        for graph in graphs {
            index += 1;
            BaWriter::write_graph_ba(graph, (index) as u32, &mut file)?;
        }
        Ok(())
    }

    fn append_graphs_to_file(graphs: &Vec<G>, path: impl AsRef<Path>) -> Result<()> {
        let file = OpenOptions::new().read(true).open(&path)?;
        let count_preface = get_graphs_count_with_preface(&file)?;
        let mut count = count_preface.0;
        let preface = count_preface.1;
        let new_count = count + graphs.len();
        BaWriter::<G>::update_graphs_count(&path, new_count, preface)?;
        let mut file = OpenOptions::new().append(true).open(&path)?;
        for graph in graphs {
            count += 1;
            BaWriter::write_graph_ba(graph, (count) as u32, &mut file)?;
        }
        Ok(())
    }

    fn update_graphs_count(
        path: impl AsRef<Path>,
        new_count: usize,
        preface: String,
    ) -> Result<()> {
        let file = OpenOptions::new().write(true).open(path)?;
        let mut writer = LineWriter::new(file);
        writer.write_all(preface.as_bytes())?;
        let count_str = format!("{}", new_count);
        writer.write_all(count_str.as_bytes())?;
        Ok(())
    }
}
