use crate::graph::graph::Graph;
use crate::service::io::error::WriteError;

pub trait GraphWriter<G>
where
    G: Graph,
{
    fn write(graph: &G) -> Result<(), WriteError>;

    // append??
}
