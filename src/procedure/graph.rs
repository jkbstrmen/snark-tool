
pub trait Graph{
    fn add_edge(&mut self, string: &str);
}

#[derive(Debug)]
pub struct SimpleGraph{
    pub graph: String
}

impl Graph for SimpleGraph {
    fn add_edge(&mut self, string: &str) {
        self.graph.push_str(string);
    }
}