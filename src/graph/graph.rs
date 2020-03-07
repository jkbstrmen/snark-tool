pub trait Graph {
    fn add_edge(&mut self, string: &str);
    fn from_str(source: &str) -> Self;
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
        SimpleGraph{graph: String::from(source)}
    }
}
