use crate::graph::graph;
use crate::service::io::error::WriteError;
use std::{io, marker, result};

type Result<T> = result::Result<T, WriteError>;

pub struct AdjWriter<G> {
    _ph: marker::PhantomData<G>,
}

impl<G> AdjWriter<G>
where
    G: graph::Graph,
{
    pub fn write_graph(graph: &G, buffer: &mut impl io::Write) -> Result<()> {
        for row in 0..graph.size() {
            for column in 0..graph.size() {
                let mut value = 0;
                if graph.has_edge(column, row) {
                    value = 1;
                }
                write!(buffer, "{}", value)?;
            }
            writeln!(buffer)?;
        }
        Ok(())
    }
}
