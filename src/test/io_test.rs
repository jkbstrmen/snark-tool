#[cfg(test)]
mod io_tests {
    use crate::graph::undirected::simple_graph::SimpleGraph;
    use crate::service::io::error::WriteError;
    use crate::service::io::reader::Reader;
    use crate::service::io::reader_ba::BaReader;
    use crate::service::io::reader_g6::G6Reader;
    use crate::service::io::writer_g6::to_g6_size;
    use crate::service::io::writer_s6::S6Writer;
    use crate::service::io::{reader_g6, reader_s6};
    use std::fs::{File, OpenOptions};
    use std::io::{self, BufRead, Write};
    use std::path::Path;
    // use crate::service::io::writer_g6::G6Writer;
    // use crate::procedure::basic_impl::basic_properties::BasicProperties;
    // use crate::test::test_data;
    // use std::slice;

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    #[test]
    fn should_read_g6() {
        // let graph_path = "resources/graphs/graphG6.g6";
        let graph_path = "resources/graphs/petersen.g6";

        if let Ok(lines) = read_lines(graph_path) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(line_str) = line {
                    println!("{}", line_str);
                    // let error = "Wrong g6 format";
                    let graph = G6Reader::<SimpleGraph>::read_graph(line_str.as_str());
                    println!("{}", graph.unwrap());

                    // print_graph(graph.unwrap());
                    // assert
                }
            }
        }
    }

    #[test]
    fn should_read_ba() {
        // let graph_path = "resources/graphs/5flow28.28";
        let graph_path = "resources/graphs/DSXDS.ALL";

        // if let Ok(lines) = read_lines(graph_path) {
        //     // Consumes the iterator, returns an (Optional) String
        //
        //     // let graph = read_graph_ba(lines);
        // }

        let file = OpenOptions::new().read(true).open(graph_path).unwrap();

        let mut reader = BaReader::<SimpleGraph>::new(&file);

        let mut index = 0;
        loop {
            let graph = reader.next();
            if graph.is_none() {
                break;
            }

            if index == 100 {
                println!();
            }

            index += 1;
            println!("{}", index);
            println!("{}", graph.unwrap().unwrap());
        }
    }

    #[test]
    fn should_write_ba() -> Result<(), WriteError> {
        // move to test_data
        // let graph_string =
        //     "]C??@Q??GCCA@??Bo??C@O?C?G_E????\\?O?A??H_??@C?@??_?C???g????G??B@??C????Ag";
        let graph_path = "resources/graphs/write_ba.ALL";

        let file_result = OpenOptions::new().write(true).append(true).open(graph_path);

        match file_result {
            Ok(mut file) => {
                // let graph = read_graph(graph_string);
                // write graphs count
                writeln!(file, "1")?;

                // TODO
                // write_graph_ba(graph.unwrap(), 1, &mut file);

                // assert
            }
            _ => {}
        }
        Ok(())
    }

    #[test]
    fn should_append_ba() {
        // let graph_path = "resources/graphs/append_ba.ALL";
        // let graph_string =
        //     "]C??@Q??GCCA@??Bo??C@O?C?G_E????\\?O?A??H_??@C?@??_?C???g????G??B@??C????Ag";
        // let graph = read_graph(graph_string);

        // TODO
        // append_graph_ba_to_file(graph.unwrap(), graph_path);

        // assert
    }

    #[test]
    fn should_write_g6() {
        // move to test_data
        // let graph_string =
        //     "]C??@Q??GCCA@??Bo??C@O?C?G_E????\\?O?A??H_??@C?@??_?C???g????G??B@??C????Ag";
        // let graph = reader_g6::G6Reader::read_graph(graph_string);

        // let mut w = Vec::new();
        // write_graph(graph.unwrap(), &mut w);
        // assert_eq!((graph_string.to_owned() + "\n").as_bytes(), &w[..]);
    }

    #[test]
    fn should_code_size() {
        let res = to_g6_size(30);
        assert_eq!(res, "]");

        let res = to_g6_size(12345);
        assert_eq!(res, "~B?x");

        let res = to_g6_size(460175067);
        assert_eq!(res, "~~?ZZZZZ");
    }

    #[test]
    fn should_write_s6() {
        let graph_path = "resources/graphs/petersen.g6";

        if let Ok(lines) = read_lines(graph_path) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(line_str) = line {
                    println!("{}", line_str);

                    let graph = reader_g6::G6Reader::<SimpleGraph>::read_graph(line_str);
                    //
                    //
                    let mut w = Vec::new();
                    S6Writer::write_graph(&graph.unwrap(), &mut w).unwrap();
                    //
                    let string = String::from_utf8(w).unwrap();
                    println!("{:?}", string);
                }
            }
        }
    }

    #[test]
    fn should_read_graph_s6() {
        let petersen_s6 = ":IG?SPc_EOrOFCQN";
        // assert

        // let test_graph = ":Fa@x^";
        // let graph_res = reader_s6::read_graph(test_graph);

        // assert
        let graph_res = reader_s6::S6Reader::<SimpleGraph>::read_graph(petersen_s6);
        println!("{}", graph_res.unwrap());
    }

    #[test]
    fn should_read_graph_g6_from_string() {
        // let graph_g6 = "]?@G@U?OK?GP?CD?o???@G???AX??__????G???g_????CG???C???B_??GO??@PAA???A_??G";
        // let graph_g6 = "]C@O?SAGC??P??O@o?Q?`????aGO????SK???O?O?OC???F??A??C??c???O@??@K???????@W";
        // let graph_res = reader_g6::G6Reader::<SimpleGraph>::read_graph(graph_g6);
        // print_graph(graph_res.unwrap());
    }

    // temp
    #[test]
    fn test() {
        // let mut graphs = vec![];
        // let properties = BasicProperties { colorable: false };
        // graphs.push((test_data::get_petersen_graph(), properties));
        //
        //
        // G6Writer::write_graphs_to_file_2(GraphsIter::new(&graphs), "test.test");
    }

    // struct GraphsIter<'a, G> {
    //     next: slice::Iter<'a, (G, BasicProperties)>
    // }
    //
    // impl<'a, G> Iterator for GraphsIter<'a, G> {
    //     type Item = &'a G;
    //
    //     fn next(&mut self) -> Option<Self::Item> {
    //         let next = self.next.next();
    //         if next.is_none() {
    //             return None;
    //         }
    //         Some(&next.unwrap().0)
    //     }
    // }
    //
    // impl<'a, G> GraphsIter<'a, G> {
    //     pub fn new(graphs: &'a Vec<(G, BasicProperties)>) -> Self {
    //         GraphsIter{
    //             next: graphs.iter()
    //         }
    //     }
    // }
}
