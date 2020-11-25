#[cfg(test)]
pub mod constructions_tests {
    use crate::graph::edge::{Edge, EdgeConstructor};
    use crate::graph::graph::{Graph, GraphConstructor};
    use crate::graph::undirected::edge::UndirectedEdge;
    use crate::graph::undirected::simple_graph::SimpleGraph;
    use crate::graph::undirected::vertex::SimpleVertex;
    use crate::graph::undirected_sparse::graph::SimpleSparseGraph;
    use crate::graph::undirected_sparse::vertex::VertexWithEdges;
    use crate::graph::vertex::Vertex;
    use crate::service::colour::colouriser::Colourizer;
    use crate::service::colour::sat::SATColourizer;
    use crate::service::io::reader_g6::G6Reader;
    use crate::service::io::writer_g6::G6Writer;
    use crate::test::test_data::test_data;

    #[test]
    fn dot_product_test() {
        // input two graphs G and H
        // take two non adjacent edges of G and remove them
        // take two adjacent vertices {x, y} of H and remove them along with edges of these vertices
        // connect each vertex of order 2 of graph G with vertex of order 2 of graph H

        let graph_g = G6Reader::<SimpleSparseGraph>::read_graph(test_data::SNARK_IN_G6_36).unwrap();
        let graph_h = G6Reader::<SimpleSparseGraph>::read_graph(test_data::SNARK_IN_G6_40).unwrap();

        // let graph_g =
        //     G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_10_PETERSEN).unwrap();
        // let graph_h =
        //     G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_10_PETERSEN).unwrap();
        // let graph_g =
        //     G6Reader::<SimpleSparseGraph>::read_graph(test_data::SNARK_IN_G6_10_PETERSEN).unwrap();
        // let graph_h =
        //     G6Reader::<SimpleSparseGraph>::read_graph(test_data::SNARK_IN_G6_10_PETERSEN).unwrap();

        let gh = dot_product_first(&graph_g, &graph_h);

        println!("{:}", gh);

        let colourable = SATColourizer::is_colorable(&gh);
        println!("colourable: {}", colourable);
    }

    // TODO - refactor and use with Graph trait
    fn dot_product_first(
        graph_g: &SimpleSparseGraph,
        graph_h: &SimpleSparseGraph,
        // first_edge_of_g: &UndirectedEdge,
        // second_edge_of_g: &UndirectedEdge,
        // first_vertex_of_h: &SimpleVertex,
        // second_vertex_of_h: &SimpleVertex,
    ) -> SimpleSparseGraph {
        // add graph_h to graph_g
        let mut graph_gh = graph_g.clone();
        let graph_h_begin_index = graph_g.size();
        for vertex in 0..graph_h.size() {
            graph_gh.add_vertex();
        }
        for edge in graph_h.edges() {
            graph_gh.add_edge(
                graph_h_begin_index + edge.from(),
                graph_h_begin_index + edge.to(),
            );
        }

        // take two non adjacent edges of G and remove them
        let mut first_edge: Option<&UndirectedEdge> = None;
        for edge in graph_g.edges() {
            first_edge = Some(edge);
            break;
        }

        if first_edge.is_none() {
            // TODO - handle
            panic!("first edge is none");
        }
        let first_edge = first_edge.unwrap();

        // find second edge - non adjacent to first
        let mut second_edge: Option<&UndirectedEdge> = None;
        for edge in graph_g.edges() {
            if !edge.is_adjacent(&first_edge) {
                second_edge = Some(edge);
                break;
            }
        }
        if second_edge.is_none() {
            // TODO - handle
            panic!("second edge is none");
        }
        let second_edge = second_edge.unwrap();
        graph_gh.remove_edge(first_edge.from(), first_edge.to());
        graph_gh.remove_edge(second_edge.from(), second_edge.to());

        // take two adjacent vertices {x, y} of H and remove them along with edges of these vertices
        let first_neighbor_of_x;
        let second_neighbor_of_x;
        let mut first_neighbor_of_y = 0;
        let mut second_neighbor_of_y = 0;

        let mut vertex_x: Option<&VertexWithEdges> = None;
        for vertex in graph_h.vertices() {
            vertex_x = Some(vertex);
            break;
        }
        if vertex_x.is_none() {
            // TODO - handle
            panic!("second edge is none");
        }

        let vertex_x = vertex_x.unwrap();
        let y_index = *(vertex_x.neighbors().get(0).unwrap());
        let vertex_y = &graph_h.vertices[*(vertex_x.neighbors().get(0).unwrap())].index();
        first_neighbor_of_x = graph_h.vertices[*(vertex_x.neighbors().get(1).unwrap())]
            .index()
            .clone();
        second_neighbor_of_x = graph_h.vertices[*(vertex_x.neighbors().get(2).unwrap())]
            .index()
            .clone();

        let vertex_x = vertex_x.index();
        let mut neighbors_of_y = graph_h.vertices[*vertex_y].neighbors().clone();
        neighbors_of_y.retain(|vertex| *vertex != vertex_x);
        first_neighbor_of_y = neighbors_of_y[0];
        second_neighbor_of_y = neighbors_of_y[1];

        graph_gh.remove_edges_of_vertex(vertex_x + graph_h_begin_index);
        graph_gh.remove_edges_of_vertex(vertex_y + graph_h_begin_index);

        // connect each vertex of order 2 of graph G with vertex of order 2 of graph H
        graph_gh.add_edge(first_edge.from(), first_neighbor_of_x + graph_h_begin_index);
        graph_gh.add_edge(first_edge.to(), second_neighbor_of_x + graph_h_begin_index);
        graph_gh.add_edge(
            second_edge.from(),
            first_neighbor_of_y + graph_h_begin_index,
        );
        graph_gh.add_edge(second_edge.to(), second_neighbor_of_y + graph_h_begin_index);

        // println!("{:?}", first_edge);
        // println!("{:?}", second_edge);
        // println!("{}", vertex_x + graph_h_begin_index);
        // println!("{}", vertex_y + graph_h_begin_index);
        //
        // let final_g6 = G6Writer::graph_to_g6_string(&graph_gh);
        // println!("{}", final_g6);

        graph_gh

        // would has to be reindexed
        // let mut final_graph = SimpleSparseGraph::with_vertices_capacity(graph_gh.size());
        // for edge in graph_gh.edges() {
        //     final_graph.add_edge(edge.from(), edge.to());
        // }
        // final_graph
    }

    fn dot_product(
        graph_g: &SimpleGraph,
        graph_h: &SimpleGraph,
        first_edge_of_g: &UndirectedEdge,
        second_edge_of_g: &UndirectedEdge,
        first_vertex_of_h: &SimpleVertex,
        second_vertex_of_h: &SimpleVertex,
    ) -> SimpleGraph {
        unimplemented!()
    }

    #[test]
    fn i_extension_test() {
        // we need two non adjacent removable edges
        // foreach edge
        //      remove edge, between vertices of edge insert new vertex, connect new vertex with existing vertices
        // connect both new vertices with each other
    }

    #[test]
    fn y_extension_test() {}

    #[test]
    fn two_i_extension_test() {}

    #[test]
    fn is_isomorphic_test() {
        // petgraph?
    }
}
