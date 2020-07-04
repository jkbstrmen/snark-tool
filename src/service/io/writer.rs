use crate::graph::graph::Graph;
use crate::service::io::error::WriteError;

// not used yet
pub trait GraphWriter<G>
where
    G: Graph,
{
    fn write(graph: &G) -> Result<(), WriteError>;
}
