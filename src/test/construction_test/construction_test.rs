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
    use crate::service::colour::colouriser::Colouriser;
    use crate::service::colour::dfs_improved::DFSColourizer;
    use crate::service::colour::sat::SATColourizer;
    use crate::service::component_analysis::removable_edge::{
        removable_edges, RemovablePairsOfEdges,
    };
    use crate::service::constructions::dot_product::dot_product_first;
    use crate::service::constructions::i_extension::{i_extension, IExtensions};
    use crate::service::constructions::y_extension::y_extension;
    use crate::service::io::reader_g6::G6Reader;
    use crate::service::io::writer_g6::G6Writer;
    use crate::test::test_data::test_data;

    #[test]
    fn dot_product_test() {
        // input two graphs G and H
        // take two non adjacent edges of G and remove them
        // take two adjacent vertices {x, y} of H and remove them along with edges of these vertices
        // connect each vertex of order 2 of graph G with vertex of order 2 of graph H

        // let graph_g = G6Reader::<SimpleSparseGraph>::read_graph(test_data::SNARK_IN_G6_36).unwrap();
        // let graph_h = G6Reader::<SimpleSparseGraph>::read_graph(test_data::SNARK_IN_G6_40).unwrap();

        // let graph_g =
        //     G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_10_PETERSEN).unwrap();
        // let graph_h =
        //     G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_10_PETERSEN).unwrap();
        let graph_g =
            G6Reader::<SimpleSparseGraph>::read_graph(test_data::SNARK_IN_G6_10_PETERSEN).unwrap();
        let graph_h =
            G6Reader::<SimpleSparseGraph>::read_graph(test_data::SNARK_IN_G6_10_PETERSEN).unwrap();

        let gh = dot_product_first(&graph_g, &graph_h);

        println!("{:}", gh);
        let final_g6 = G6Writer::graph_to_g6_string(&gh);
        println!("{}", final_g6);

        //
        // let colourable = SATColourizer::is_colorable(&gh);
        // println!("colourable: {}", colourable);
    }

    #[test]
    fn i_extension_test() {
        let graph_g =
            G6Reader::<SimpleSparseGraph>::read_graph(test_data::SNARK_IN_G6_10_PETERSEN).unwrap();

        let first_edge = UndirectedEdge::new(0, 4);
        let second_edge = UndirectedEdge::new(3, 5);
        let extended = i_extension(&graph_g, &first_edge, &second_edge);

        let final_g6 = G6Writer::graph_to_g6_string(&extended);
        println!("{}", final_g6);
    }

    #[test]
    fn i_extension_iterator_test() {
        let graph_g =
            G6Reader::<SimpleSparseGraph>::read_graph(test_data::SNARK_IN_G6_10_PETERSEN).unwrap();

        let colouriser = DFSColourizer::new();
        let i_extensions = IExtensions::new(&graph_g, &colouriser);

        for i_extension in i_extensions {
            let colourable = DFSColourizer::is_colorable(&i_extension);
            println!("{}", colourable);

            // let final_g6 = G6Writer::graph_to_g6_string(&extended);
            // println!("{}", final_g6);
        }
    }

    #[test]
    fn y_extension_test() {
        let graph_g =
            G6Reader::<SimpleSparseGraph>::read_graph(test_data::SNARK_IN_G6_10_PETERSEN).unwrap();

        let first_edge = UndirectedEdge::new(0, 4);
        let second_edge = UndirectedEdge::new(3, 5);
        let third_edge = UndirectedEdge::new(8, 9);
        let extended = y_extension(&graph_g, &first_edge, &second_edge, &third_edge);

        let final_g6 = G6Writer::graph_to_g6_string(&extended);
        println!("{}", final_g6);
    }

    #[test]
    fn two_i_extension_test() {}

    #[test]
    fn is_isomorphic_test() {
        // petgraph?
    }

    #[test]
    fn removable_edges_test() {
        let graph = G6Reader::<SimpleSparseGraph>::read_graph(test_data::SNARK_IN_G6_10_PETERSEN) // SNARK_IN_G6_30_ACRITICAL_1
            .unwrap();

        // let edges = removable_edges(&graph);

        let colouriser = DFSColourizer::new();
        let pairs = RemovablePairsOfEdges::new(&graph, &colouriser);
        for pair in pairs {
            println!("{:?}", pair);
        }
    }
}
