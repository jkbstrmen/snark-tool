pub trait Graph {
    fn add_edge(&mut self, string: &str);
    fn from_str(source: &str) -> Self;

    // has_edge
    // edge iterator
    // vertex iterator
    // remove edge
    // remove vertex
    // edges_vec
    // vertices_vec
    // ...

    // update_edge
    // update_vertex
}

#[derive(Debug)]
pub struct SimpleGraph {
    pub graph: String,
}

impl Graph for SimpleGraph {
    fn add_edge(&mut self, string: &str) {
        self.graph.push_str(string);
    }

    fn from_str(source: &str) -> Self {
        SimpleGraph {
            graph: String::from(source),
        }
    }
}
