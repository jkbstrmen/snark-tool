#[cfg(test)]
mod io_tests {
    use std::fs::OpenOptions;

    use crate::graph::undirected::simple_graph::graph::SimpleGraph;
    use crate::service::io::reader::Reader;
    use crate::service::io::reader_ba::BaReader;
    use crate::service::io::reader_g6::G6Reader;
    use crate::service::io::reader_s6::S6Reader;
    //use crate::service::io::writer_adj::AdjWriter;
    use crate::service::io::writer_ba::BaWriter;
    use crate::service::io::writer_g6::G6Writer;
    use crate::service::io::writer_s6::S6Writer;
    use crate::tests::test_data::test_data;

    /// READERS
    #[test]
    fn should_read_g6() {
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_10_PETERSEN);
        assert_eq!(graph.unwrap(), test_data::get_petersen_graph());
    }

    #[test]
    fn should_read_s6() {
        let graph = S6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_S6_10_PETERSEN);
        assert_eq!(graph.unwrap(), test_data::get_petersen_graph());
    }

    #[test]
    fn should_read_ba() {
        let file = OpenOptions::new()
            .read(true)
            .open("resources/graphs/petersen.10")
            .unwrap();
        let mut reader = BaReader::<SimpleGraph>::new(&file);
        let graph = reader.next().unwrap().unwrap();
        assert_eq!(graph, test_data::get_petersen_graph());
    }

    #[test]
    fn should_compare_readers() {
        let g6 = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_10_PETERSEN).unwrap();
        let s6 = S6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_S6_10_PETERSEN).unwrap();
        let file = OpenOptions::new()
            .read(true)
            .open("resources/graphs/petersen.10")
            .unwrap();
        let mut reader = BaReader::<SimpleGraph>::new(&file);
        let ba = reader.next().unwrap().unwrap();
        assert_eq!(g6, s6);
        assert_eq!(g6, ba);
        assert_eq!(s6, ba);
    }

    /// WRITERS
    #[test]
    fn should_write_g6() {
        let mut target = Vec::new();
        let result = G6Writer::write_graph(&test_data::get_petersen_graph(), &mut target);
        let graph_string = String::from_utf8(target).unwrap();
        assert_eq!(result.is_ok(), true);
        assert_eq!(
            graph_string,
            format!("{}\n", test_data::SNARK_IN_G6_10_PETERSEN)
        );
    }

    #[test]
    fn should_write_s6() {
        let mut target = Vec::new();
        let result = S6Writer::write_graph(&test_data::get_petersen_graph(), &mut target);
        let graph_string = String::from_utf8(target).unwrap();
        assert_eq!(result.is_ok(), true);
        assert_eq!(
            graph_string,
            format!("{}\n", test_data::SNARK_IN_S6_10_PETERSEN)
        );
    }

    #[test]
    fn should_write_ba() {
        let mut target = Vec::new();
        let result = BaWriter::write_graph_ba(&test_data::get_petersen_graph(), 1, &mut target);
        let graph_string = String::from_utf8(target).unwrap();
        assert_eq!(result.is_ok(), true);
        assert_eq!(graph_string, test_data::SNARK_IN_BA_10_PETERSEN);
    }

    // #[test]
    // fn should_write_adj_matrix() {
    //     let mut target = Vec::new();
    //     let result = AdjWriter::write_graph(&test_data::get_petersen_graph(), &mut target);
    //     let graph_string = String::from_utf8(target).unwrap();
    //     assert_eq!(result.is_ok(), true);
    //     assert_eq!(graph_string, test_data::SNARK_IN_ADJ_10_PETERSEN);
    // }

    #[test]
    fn should_read_g6_write_s6() {
        let mut target = Vec::new();
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_112);
        let result = S6Writer::write_graph(&graph.unwrap(), &mut target);
        let graph_string = String::from_utf8(target).unwrap();
        assert_eq!(result.is_ok(), true);
        assert_eq!(
            graph_string,
            format!("{}{}", test_data::NO_SNARK_IN_S6_112, "\n")
        );
    }

    #[test]
    fn should_read_g6_s6_same_graph() {
        let graph_s6 = S6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_S6_112);
        let graph_g6 = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_112);
        assert_eq!(graph_s6.unwrap(), graph_g6.unwrap());
    }

    // #[test]
    // fn should_read_json() {
    //     let file = OpenOptions::new().read(true).open("output").unwrap();
    //
    //     let mut reader = JsonReader::<SimpleGraph>::new(&file);
    //     reader.next();
    // }
}
