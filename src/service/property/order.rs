use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;

// pub fn order<G: Graph>(graph: G) -> result::Result<usize, Error> {
//     Ok(0)
// }

#[allow(dead_code)]
pub fn is_cubic<G: Graph>(graph: G) -> bool {
    let mut is_cubic = true;
    for vertex in graph.vertices() {
        let edges = graph.edges_of_vertex(vertex.index());
        let mut edges_count = 0;
        for _edge in edges {
            edges_count += 1;
        }
        if edges_count != 3 {
            is_cubic = false;
        }
    }
    is_cubic
}
