
pub trait Graph<V = SimpleVertex, E = UndirectedEdge> where E: Edge{
    fn add_edge(&mut self, edge: E) ;

    // temp
    // fn from_str(source: &str) -> Self;

    // size
    // has_edge
    // edge_iter = edge_iterator
    // vertex_iter = vertex_iterator
    // add_vertex
    // remove_edge
    // remove_vertex
    // edges = edges of vertex
    // vertex = vertex with index
    // vertex_mut

    // edges_count
    // vertices_count
    // update_edge
    // update_vertex

    // CONSTRUCTORS
    fn with_capacity(vertices: usize, edges: usize) -> Self;
    // with_vertices_capacity
    // with_edes_capacity

    // ??
    // is_directed
}

pub trait GraphStructures {
    // ??
    // edges_vec
    // vertices_vec
    // adj_matrix
    // ...
}

pub trait Edge {

    // from
    // to
    // colour? or weight

    // CONSTRUCTORS
}
pub struct UndirectedEdge {}
impl Edge for UndirectedEdge {}

pub trait Vertex {
    // index
    // weight

    // CONSTRUCTORS
}

pub struct SimpleVertex {

}

// struct Edge {
//     pub from: u32,
//     pub to: u32,
// }

//
//
//


#[derive(Debug)]
pub struct SimpleGraph {
    pub graph: String,
}

/// undirected, without loop, without multiple edges
impl Graph for SimpleGraph {
    fn add_edge(&mut self, edge: UndirectedEdge) {
        // self.graph.push_str(string);
    }

    fn with_capacity(vertices: usize, edges: usize) -> Self {
        // let a = V;
        SimpleGraph{ graph: "".to_string() }
    }

    // fn from_str(source: &str) -> Self {
    //     SimpleGraph {
    //         graph: String::from(source),
    //     }
    // }
}
