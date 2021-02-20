#[cfg(test)]
pub mod colour_tests {
    use crate::graph::undirected::simple_graph::graph::SimpleGraph;
    use crate::service::colour::colouriser::Colouriser;
    use crate::service::colour::dfs_improved::DFSColourizer;
    use crate::service::colour::sat::SATColourizer;
    use crate::service::colour::sat_new::SATColourizerNew;
    use crate::service::colour::sat_splr::SATSplrColourizer;
    use crate::service::io::reader_g6::G6Reader;
    use crate::service::io::reader_s6::S6Reader;
    use crate::test::test_data::test_data;
    use std::time::Instant;
    use {splr::*, std::convert::TryFrom};

    // #[test]
    // fn test() {
    //     let v: Vec<Vec<i32>> = vec![vec![1, 2], vec![-1, 3], vec![1, -3], vec![-1, 2]];
    //     let result = match Certificate::try_from(v).expect("panic!") {
    //         Certificate::UNSAT => 0,
    //         Certificate::SAT(vec) => vec.len(),
    //     };
    //
    //     println!("{}", result);
    // }

    #[test]
    fn should_colour_using_sat() {
        let graph = test_data::get_petersen_graph();
        let colourable = SATSplrColourizer::is_colorable(&graph);
        assert_eq!(colourable, false);

        let graph = test_data::get_falcon_graph();
        let colourable = SATSplrColourizer::is_colorable(&graph);
        assert_eq!(colourable, false);

        let graph_g6 = test_data::SNARK_IN_S6_76_3TF1_03;
        let graph: SimpleGraph = S6Reader::read_graph(graph_g6).unwrap();
        let colourable = SATSplrColourizer::is_colorable(&graph);
        assert_eq!(colourable, false);
    }

    #[test]
    fn new_formula() {
        // let combinations = BinaryCombinationsIterator::new(9);
        // for combination in combinations {
        //     println!("{:?}", combination)
        // }

        let begin = Instant::now();

        // let graph_g6 = test_data::SNARK_IN_S6_76_3TF1_03;
        // let graph: SimpleGraph = S6Reader::read_graph(graph_g6).unwrap();
        let graph_g6 = test_data::NO_SNARK_IN_G6_18;
        let graph: SimpleGraph = G6Reader::read_graph(graph_g6).unwrap();
        // let colourable = SATColourizerNew::is_colorable(&graph);
        // let colourable = SATColourizer::is_colorable(&graph);
        let colourable = SATColourizerNew2::is_colorable(&graph);
        assert_eq!(colourable, true);

        println!("{}", begin.elapsed().as_millis())
    }

    use crate::service::colour::sat_new_2::SATColourizerNew2;
    use itertools::Itertools;

    #[test]
    fn temp() {
        // let it = (1..4).combinations_with_replacement(3);
        // for i in it {
        //     println!("{:?}", i);
        // }

        let it = (1..4).permutations(3);
        for i in it {
            println!("{:?}", i);
        }
    }
}
