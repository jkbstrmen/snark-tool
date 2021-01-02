#[cfg(test)]
pub mod measurement_tests {
    use crate::graph::graph::Graph;
    use crate::graph::undirected::simple_graph::graph::SimpleGraph;
    use crate::service::chromatic_properties::stable_and_critical_prop::StableAndCriticalProperties;
    use crate::service::colour::colouriser::Colouriser;
    use crate::service::colour::cvd;
    use crate::service::colour::cvd_dfs::CvdDfsColourizer;
    use crate::service::colour::dfs_improved::DFSColourizer;
    use crate::service::colour::dfs_orig::DFSColourizerOriginal;
    use crate::service::colour::sat::SATColourizer;
    use crate::service::io::reader::Reader;
    use crate::service::io::reader_g6::G6Reader;
    use crate::service::io::reader_s6::S6Reader;
    use crate::service::io::writer_s6::S6Writer;
    use std::io::Write;
    use std::{fs, time};

    // pub static mut FIRST_VERTEX: u128 = 0;

    #[test]
    fn dfs_colouriser_performance() {
        // let path = "resources/measurement_samples/graph.g6";
        // let path = "resources/measurement_samples/10_28vert_snarks.g6";
        // let path = "resources/measurement_samples/Generated_graphs.28.04.sn.cyc4.10K.g6";
        // let path = "resources/measurement_samples/Generated_graphs.30.04.sn.cyc4.10K.g6";
        // let path = "resources/measurement_samples/Generated_graphs.30.04.sn.cyc4.g6";
        // let path = "resources/measurement_samples/Generated_graphs.32.04.sn.cyc4.10K.g6";
        // let path = "resources/measurement_samples/Generated_graphs.34.04.sn.cyc4.10K.g6";
        // let path = "resources/measurement_samples/Generated_graphs.36.04.sn.cyc4.10K.g6";
        // let path = "resources/measurement_samples/Generated_graphs.38.05.sn.cyc4.10K.g6";
        // let path = "resources/measurement_samples/Generated_graphs.40.10K.g6";
        // let path = "resources/measurement_samples/Generated_graphs.44.10K.g6";

        // let path = "resources/measurement_samples/100K.Generated_graphs.30.04.sn.cyc4.g6";
        // let path = "resources/measurement_samples/100K.Generated_graphs.32.04.sn.cyc4.g6";
        // let path = "resources/measurement_samples/100K.Generated_graphs.34.04.sn.cyc4.g6";
        // let path = "resources/measurement_samples/100K.Generated_graphs.36.04.sn.cyc4.g6";
        let path = "resources/measurement_samples/100K.Generated_graphs.38.05.sn.cyc4.g6";

        let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();
        let mut temp_file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open("temp")
            .unwrap();
        let mut reader = G6Reader::<SimpleGraph>::new(&file_result);

        let begin = time::Instant::now();

        let mut counter = 0;
        let mut all_false = true;
        while let Some(graph_result) = reader.next() {
            let mut graph = graph_result.unwrap();
            // graph.remove_edges_of_vertex(2);
            // graph.remove_edges_of_vertex(15);

            // let colourable = DFSColourizerSimple::is_colorable(&graph);
            // let colourable = DFSColourizerOriginal::is_colorable(&graph);
            let colourable = DFSColourizer::is_colorable(&graph);

            // let colourable = DFSColourizerNaive::is_colorable(&graph);
            // let colourable = SATColourizer::is_colorable(&graph);
            if colourable {
                all_false = false;
            }

            writeln!(temp_file, "{}", counter);
            counter += 1;
        }
        println!("all false: {}", all_false);
        println!("elapsed: {}", begin.elapsed().as_millis());
    }

    #[test]
    fn dfs_colouriser_critical_stable_properties_performance() {
        // let path = "resources/measurement_samples/10_28vert_snarks.g6";
        // let path = "resources/measurement_samples/10_30vert_snarks.g6";
        // let path = "resources/measurement_samples/10_32vert_snarks.g6";
        // let path = "resources/measurement_samples/10_34vert_snarks.g6";
        // let path = "resources/measurement_samples/10_38vert_snarks.g6";
        // let path = "resources/measurement_samples/10_40vert_snarks.g6";
        // let path = "resources/measurement_samples/10_44vert_snarks.g6";

        // let path = "resources/measurement_samples/10_36vert_snarks_stable.g6";
        // let path = "resources/measurement_samples/10_36vert_snarks_bicritical.g6";
        // let path = "resources/measurement_samples/10_36vert_snarks_str_critical.g6";

        // let path = "resources/measurement_samples/100_28vert_snarks.g6";
        // let path = "resources/measurement_samples/100_30vert_snarks.g6";
        // let path = "resources/measurement_samples/100_32vert_snarks.g6";
        let path = "resources/measurement_samples/100_34vert_snarks.g6";
        // let path = "resources/measurement_samples/100_36vert_snarks.g6";
        // let path = "resources/measurement_samples/100_38vert_snarks.g6";
        let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();
        let mut reader = G6Reader::<SimpleGraph>::new(&file_result);

        let begin = time::Instant::now();

        let mut crit_count = 0;
        let mut cocrit_count = 0;
        let mut v_sub_count = 0;
        let mut e_sub_count = 0;
        let mut stab_count = 0;
        let mut costab_count = 0;
        while let Some(graph_result) = reader.next() {
            let graph = graph_result.unwrap();

            let mut props =
                // StableAndCriticalProperties::of_graph_with_colourizer(&graph, CvdDfsColourizer::new());
            StableAndCriticalProperties::of_graph_with_colourizer(&graph, DFSColourizerOriginal::new());
            // StableAndCriticalProperties::of_graph_with_colourizer(&graph, DFSColourizer::new());
            // StableAndCriticalProperties::of_graph_with_colourizer(&graph, DFSColourizerSimple::new());
            // StableAndCriticalProperties::of_graph_with_colourizer(&graph, DfsDfsColourizer::new());
            let crit = props.is_critical();
            let cocrit = props.is_cocritical();
            let e_subcrit = props.is_edge_subcritical();
            let v_subcrit = props.is_vertex_subcritical();
            let stable = props.is_stable();
            let costable = props.is_costable();

            if crit {
                crit_count += 1;
            }
            if cocrit {
                cocrit_count += 1;
            }
            if v_subcrit {
                v_sub_count += 1;
            }
            if e_subcrit {
                e_sub_count += 1;
            }
            if stable {
                stab_count += 1;
            }
            if costable {
                costab_count += 1;
            }
        }

        println!("CRITICAL: {}", crit_count);
        println!("COCRITICAL: {}", cocrit_count);
        println!("V subCRITICAL: {}", v_sub_count);
        println!("E subCRITICAL: {}", e_sub_count);
        println!("STABLE: {}", stab_count);
        println!("COSTABLE: {}", costab_count);

        println!("elapsed: {}", begin.elapsed().as_millis());

        // println!("elapsed on CVD: {}ms", unsafe { ELAPSED / 1000000 });
        // println!("elapsed on DFS: {}ms", unsafe { ELAPSED_0 / 1000000 });
        // println!("elapsed partial2: {}ms", unsafe { ELAPSED_2 / 1000000 });
        // println!("elapsed partial3: {}ms", unsafe { ELAPSED_3 / 1000000 });
        //
        // unsafe {
        //     println!("all calls: {}", ALL_CALLS);
        // }
        // unsafe {
        //     println!("dfs calls: {}", DFS_CALLS);
        // }
        // println!(
        //     "{} out of {} calls were true but called DFS",
        //     unsafe { DFS_AFTER_CVD_WHEN_TRUE },
        //     unsafe { ALL_CALLS }
        // );

        // println!("available colours of vertex calls: {}", unsafe { COUNTER_3 });
        // println!(
        //     "all: {}, colourable: {}, non colourable: {}",
        //     unsafe { COUNTER_1 },
        //     unsafe { COUNTER_2 },
        //     unsafe { COUNTER_3 }
        // )
    }

    ///
    /// tuning subgraphs
    ///
    /*#[test]
    fn critical_stable_properties_subgraphs() {
        // let path = "resources/measurement_samples/100_28vert_snarks.g6";
        // let path = "resources/measurement_samples/Generated_graphs.28.04.sn.cyc4.g6";
        // let path = "resources/measurement_samples/100_30vert_snarks.g6";
        // let path = "resources/measurement_samples/100_32vert_snarks.g6";
        // let path = "resources/measurement_samples/100_34vert_snarks.g6";
        let path = "resources/measurement_samples/100_36vert_snarks.g6";

        let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();
        let mut reader = G6Reader::<SimpleGraph>::new(&file_result);

        let begin = time::Instant::now();

        let mut crit_count = 0;
        let mut cocrit_count = 0;
        let mut v_sub_count = 0;
        let mut e_sub_count = 0;
        let mut stab_count = 0;
        let mut costab_count = 0;
        let mut index = 0;
        while let Some(graph_result) = reader.next() {
            let graph = graph_result.unwrap();

            let mut props =
                // StableAndCriticalProperties::of_graph_with_colourizer(&graph, BFSColourizer_1_0::new());
                // StableAndCriticalProperties::of_graph_with_colourizer(&graph, BFSColourizer_2_0::new());
                // StableAndCriticalProperties::of_graph_with_colourizer(&graph, BFSColourizer_2_1::new());
                // StableAndCriticalProperties::of_graph_with_colourizer(&graph, BFSColourizer_2_2::new());
                // StableAndCriticalProperties::of_graph_with_colourizer(&graph, CvdDfsColourizer::new());
                StableAndCriticalProperties::of_graph_with_colourizer(&graph, DFSColourizer::new());
            let crit = props.is_critical();
            let cocrit = props.is_cocritical();
            let e_subcrit = props.is_edge_subcritical();
            let v_subcrit = props.is_vertex_subcritical();
            let stable = props.is_stable();
            let costable = props.is_costable();

            if crit {
                crit_count += 1;
                println!("index: {}", index);
            }
            if cocrit {
                cocrit_count += 1;
            }
            if v_subcrit {
                v_sub_count += 1;
            }
            if e_subcrit {
                e_sub_count += 1;
            }
            if stable {
                stab_count += 1;
            }
            if costable {
                costab_count += 1;
            }
            index += 1;
        }

        println!("CRITICAL: {}", crit_count);
        println!("COCRITICAL: {}", cocrit_count);
        println!("V subCRITICAL: {}", v_sub_count);
        println!("E subCRITICAL: {}", e_sub_count);
        println!("STABLE: {}", stab_count);
        println!("COSTABLE: {}", costab_count);

        println!("elapsed: {}ms", begin.elapsed().as_millis());

        println!("elapsed on all colourings: {}ms", unsafe {
            ELAPSED_0 / 1000000
        });
        println!("elapsed on colourable subgraphs: {}ms", unsafe {
            ELAPSED_2 / 1000000
        });
        println!("elapsed on non colourable subgraphs: {}ms", unsafe {
            ELAPSED_3 / 1000000
        });

        unsafe {
            println!("all calls: {}", ALL_CALLS);
        }
        unsafe {
            println!("true calls: {}", COUNTER_3);
        }
    }*/

    #[test]
    fn temp2() {
        let size = 100;
        let dir = "resources/measurement_samples/python-small-1st";
        let file_name = "cvd_measurement_graphs_";
        let path = format!("{}/{}{}.s6", dir, file_name, size);

        println!("{}", path);
    }
}

#[cfg(test)]
pub mod sat_measurement_tests {
    use crate::graph::graph::Graph;
    use crate::graph::undirected::simple_edge_graph::graph::SimpleEdgeGraph;
    use crate::graph::undirected::simple_graph::graph::SimpleGraph;
    use crate::service::chromatic_properties::stable_and_critical_prop::StableAndCriticalProperties;
    use crate::service::colour::colouriser::Colouriser;
    use crate::service::colour::dfs_improved::DFSColourizer;
    use crate::service::colour::sat::SATColourizer;
    use crate::service::colour::sat_splr::SATSplrColourizer;
    use crate::service::constructions::dot_product::DotProducts;
    use crate::service::io::reader::Reader;
    use crate::service::io::reader_g6::G6Reader;
    use crate::service::io::writer_g6::G6Writer;
    use crate::service::io::writer_s6::S6Writer;
    use crate::test::test_data::test_data;
    use std::io::Write;
    use std::{fs, time};

    #[test]
    fn sat_prepare_graphs() {
        // let path = "resources/measurement_samples/Generated_graphs.38.05.sn.cyc4.10K.g6";
        let path = "resources/measurement_samples/Generated_graphs.36.04.sn.cyc4.10K.g6";

        let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();
        let mut reader = G6Reader::<SimpleGraph>::new(&file_result);

        // let begin = time::Instant::now();
        let out_file_path = "resources/measurement_samples/dfs-vs-sat/10K.dot_product.56.g6";
        let mut out_file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(out_file_path)
            .unwrap();

        // let graph_2_g6 = test_data::SNARK_IN_G6_10_PETERSEN;
        let graph_2_g6 = test_data::SNARK_IN_G6_22;
        let graph_2 = G6Reader::read_graph(graph_2_g6).unwrap();

        while let Some(graph_result) = reader.next() {
            let mut graph = graph_result.unwrap();

            let mut dot_products = DotProducts::new(&graph, &graph_2);
            let extended = dot_products.next().unwrap();

            G6Writer::write_graph(&extended, &mut out_file);

            // let colourable = DFSColourizer::is_colorable(&graph);
            // let colourable = SATColourizer::is_colorable(&graph);
            // assert_eq!(colourable, false);
        }
        // println!("elapsed: {}", begin.elapsed().as_millis());
    }

    #[test]
    fn sat_colouriser_performance() {
        // let path = "resources/measurement_samples/dfs-vs-sat/10K.dot_product.56.g6";
        // let path = "resources/measurement_samples/dfs-vs-sat/10K.dot_product.58.g6";
        let path = "resources/measurement_samples/dfs-vs-sat/2K.dot_product.58.g6";

        let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();
        let mut reader = G6Reader::<SimpleGraph>::new(&file_result);

        let begin = time::Instant::now();

        while let Some(graph_result) = reader.next() {
            let mut graph = graph_result.unwrap();

            // let colourable = DFSColourizer::is_colorable(&graph);
            let colourable = SATColourizer::is_colorable(&graph);
            // let colourable = SATSplrColourizer::is_colorable(&graph);
            assert_eq!(colourable, false);

            // assert_eq!(graph.size(), 56);
        }
        println!("elapsed: {}", begin.elapsed().as_millis());
    }

    fn dfs_vs_sat_sizes() -> Vec<usize> {
        let mut size = 44;
        let mut sizes = vec![];
        while size < 56 {
            size += 2;
            sizes.push(size)
        }
        sizes
    }

    #[test]
    fn sat_colouriser_performance_set_of_files() {
        let out_file_path = "resources/measurement_samples/dfs-vs-sat/results.txt";
        let mut out_file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(out_file_path)
            .unwrap();

        for size in dfs_vs_sat_sizes() {
            let path = format!(
                "resources/measurement_samples/dfs-vs-sat/10K.dot_product.{}.g6",
                size
            );

            let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();
            let mut reader = G6Reader::<SimpleGraph>::new(&file_result);

            let begin = time::Instant::now();

            while let Some(graph_result) = reader.next() {
                let mut graph = graph_result.unwrap();

                // let colourable = DFSColourizer::is_colorable(&graph);
                let colourable = SATColourizer::is_colorable(&graph);
                assert_eq!(colourable, false);
                assert_eq!(graph.size(), size);
            }
            writeln!(
                out_file,
                "size: {}, elapsed: {} ms",
                size,
                begin.elapsed().as_millis()
            );
        }
    }

    #[test]
    fn sat_colouriser_critical_stable_properties_performance() {
        // let path = "resources/measurement_samples/10_28vert_snarks.g6";
        // let path = "resources/measurement_samples/10_30vert_snarks.g6";
        // let path = "resources/measurement_samples/10_32vert_snarks.g6";
        // let path = "resources/measurement_samples/10_34vert_snarks.g6";
        // let path = "resources/measurement_samples/10_38vert_snarks.g6";
        // let path = "resources/measurement_samples/10_40vert_snarks.g6";
        // let path = "resources/measurement_samples/10_44vert_snarks.g6";

        // let path = "resources/measurement_samples/10_36vert_snarks_stable.g6";
        // let path = "resources/measurement_samples/10_36vert_snarks_bicritical.g6";
        // let path = "resources/measurement_samples/10_36vert_snarks_str_critical.g6";

        // let path = "resources/measurement_samples/100_28vert_snarks.g6";
        // let path = "resources/measurement_samples/100_30vert_snarks.g6";
        // let path = "resources/measurement_samples/100_32vert_snarks.g6";
        let path = "resources/measurement_samples/100_34vert_snarks.g6";
        // let path = "resources/measurement_samples/100_36vert_snarks.g6";
        // let path = "resources/measurement_samples/100_38vert_snarks.g6";
        let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();
        let mut reader = G6Reader::<SimpleGraph>::new(&file_result);

        let begin = time::Instant::now();

        let mut crit_count = 0;
        let mut cocrit_count = 0;
        let mut v_sub_count = 0;
        let mut e_sub_count = 0;
        let mut stab_count = 0;
        let mut costab_count = 0;
        while let Some(graph_result) = reader.next() {
            let graph = graph_result.unwrap();

            let mut props =
                StableAndCriticalProperties::of_graph_with_colourizer(&graph, DFSColourizer::new());
            let crit = props.is_critical();
            let cocrit = props.is_cocritical();
            let e_subcrit = props.is_edge_subcritical();
            let v_subcrit = props.is_vertex_subcritical();
            let stable = props.is_stable();
            let costable = props.is_costable();

            if crit {
                crit_count += 1;
            }
            if cocrit {
                cocrit_count += 1;
            }
            if v_subcrit {
                v_sub_count += 1;
            }
            if e_subcrit {
                e_sub_count += 1;
            }
            if stable {
                stab_count += 1;
            }
            if costable {
                costab_count += 1;
            }
        }

        println!("CRITICAL: {}", crit_count);
        println!("COCRITICAL: {}", cocrit_count);
        println!("V subCRITICAL: {}", v_sub_count);
        println!("E subCRITICAL: {}", e_sub_count);
        println!("STABLE: {}", stab_count);
        println!("COSTABLE: {}", costab_count);

        println!("elapsed: {}", begin.elapsed().as_millis());
    }
}

#[cfg(test)]
pub mod cvd_measurement_tests {
    use crate::graph::graph::Graph;
    use crate::graph::undirected::simple_graph::graph::SimpleGraph;
    use crate::service::io::reader::Reader;
    use crate::service::io::reader_s6::S6Reader;
    use std::{fs, thread, time};

    #[test]
    fn temp() {
        // let path = "resources/measurement_samples/random.s6";
        // let path = "resources/measurement_samples/random_50K.s6";
        // let path = "resources/measurement_samples/random_100K.s6";
        // let path = "resources/measurement_samples/random_100.s6";
        let path = "resources/measurement_samples/random_1000.s6";
        let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();
        // let mut reader = S6Reader::<SimpleGraph>::new(&file_result);
        let mut reader = S6Reader::<SimpleGraph>::new(&file_result);

        let begin = time::Instant::now();
        let graph = reader.next().unwrap().unwrap();
        println!("reader: {}ms", begin.elapsed().as_millis());

        // println!("elapsed reading graph: {}s", begin.elapsed().as_secs());

        let begin = time::Instant::now();

        // let colourable = DFSColourizer::is_colorable(&graph);
        // let colourable = CvdDfsColourizer::is_colorable(&graph);
        // let colourable = cvd::is_colorable(&graph).unwrap();
        // let colourable = cvd_before::is_colorable(&graph).unwrap();
        let colourable = cvd::is_colorable(&graph).unwrap();
        // let colourable = SATColourizer::is_colorable(&graph);

        println!("size: {}", graph.size());
        println!("colourable: {}", colourable);
        println!("elapsed: {}s", begin.elapsed().as_secs());
        println!("elapsed: {}ns", begin.elapsed().as_nanos());
        println!(
            "elapsed: {}s",
            begin.elapsed().as_nanos() as f64 / 1000000000 as f64
        );
    }

    ///
    /// measurements of graphs randomly generated using python networkx - to compare performance
    /// with original Python CVD implementation ... needs to be run from main to run in parallel
    ///
    fn smallest_sizes() -> Vec<usize> {
        let mut size = 20;
        let mut sizes = vec![];
        while size < 90 {
            size += 10;
            sizes.push(size)
        }
        sizes
    }

    fn small_sizes() -> Vec<usize> {
        let mut size = 90;
        let mut sizes = vec![];
        while size < 200 {
            size += 10;
            sizes.push(size)
        }
        sizes
    }

    fn big_sizes() -> Vec<usize> {
        // let mut size = 0;
        let mut size = 20000;
        let mut sizes = vec![];
        while size < 100000 {
            size += 5000;
            sizes.push(size)
        }
        sizes
    }

    fn perform_measurements() {
        let number_of_iterations = 10;
        // let sizes = smallest_sizes();
        // let sizes = small_sizes();
        let sizes = big_sizes();

        // let dir = "resources/measurement_samples/python-smallest-1st";
        // let dir = "resources/measurement_samples/python-smallest-2nd";
        // let dir = "resources/measurement_samples/python-small-1st";
        // let dir = "resources/measurement_samples/python-small-2nd";
        let dir = "resources/measurement_samples/python-big-1st";
        let mut measurement_string: String = "".to_string();

        for size in sizes.iter() {
            // read file

            println!("{}", size);
            let file_name = "cvd_measurement_graphs_";
            let path = format!("{}/{}{}.s6", dir, file_name, size);
            let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();

            // let mut reader = G6Reader::<SimpleGraph>::new(&file_result);
            // let mut reader = S6Reader::<SimpleGraph>::new(&file_result);
            let mut reader = S6Reader::<SimpleGraph>::new(&file_result);

            let begin = time::Instant::now();

            let mut counter = 0;
            let mut all_true = true;
            while let Some(graph_result) = reader.next() {
                let mut graph = graph_result.unwrap();

                for number_of_iteration in 0..number_of_iterations {
                    let colourable = CvdDfsColourizer::is_colorable(&graph);
                    // let colourable = DFSColourizer::is_colorable(&graph);
                    // let colourable = SATColourizer::is_colorable(&graph);
                    if colourable {
                        all_true = false;
                    }
                }

                counter += 1;
            }

            let elapsed: f64 = (begin.elapsed().as_millis() as f64)
                / counter as f64
                / number_of_iterations as f64
                / 1000 as f64;
            // let elapsed: f64 = (begin.elapsed().as_micros() as f64) / counter as f64 / number_of_iterations as f64 / 1000 as f64;
            measurement_string =
                measurement_string.add(format!("({}, {:.3})", size, elapsed).as_str());

            let received = format!("({}, {:.4})", size, elapsed);
            let path = format!("{}/measurements_rust.txt", dir);
            let mut measurement_file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)
                .unwrap();
            // writeln!(measurement_file);
            write!(measurement_file, "{}", received);
        }

        // let path = format!("{}/{}", dir, size);
        // let mut measurement_file = fs::OpenOptions::new().create(true).write(true).open("temp").unwrap();
        // writeln!(temp_file, "{}", counter);

        println!("{}", measurement_string);
    }

    use crate::service::colour::colouriser::Colouriser;
    use crate::service::colour::cvd;
    use crate::service::colour::cvd_dfs::CvdDfsColourizer;
    use crate::service::colour::dfs_improved::DFSColourizer;
    use crate::service::colour::sat::SATColourizer;
    use std::io::Write;
    use std::ops::Add;
    use std::sync::mpsc;

    fn perform_measurements_parallel() {
        let sizes = small_sizes();
        // let sizes = big_sizes();

        let dir = "resources/measurement_samples/python-smallest-1st";
        // let dir = "resources/measurement_samples/python-small-1st";
        // let dir = "resources/measurement_samples/python-small-2nd";
        // let dir = "resources/measurement_samples/python-big-1st";
        let mut measurement_string: String = "".to_string();

        // let cpus_count = num_cpus::get();

        let (tx, rx) = mpsc::channel();
        for size in sizes.iter() {
            // read file

            println!("{}", size);
            let file_name = "cvd_measurement_graphs_";
            let path = format!("{}/{}{}.s6", dir, file_name, size);

            let tx1 = mpsc::Sender::clone(&tx);
            let size_to_move = size.clone();
            thread::spawn(move || {
                let string = perform_measurement_for_part(size_to_move, &path);
                tx1.send(string).unwrap();
            });
        }

        let path = format!("{}/measurements_rust.txt", dir);
        let mut measurement_file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .unwrap();
        writeln!(measurement_file);

        drop(tx);

        let mut measurement_string: String = "".to_string();
        for received in rx {
            println!("Got: {}", received);
            write!(measurement_file, "{}", received);
            measurement_string = measurement_string.add(&received);
        }
        println!("{}", measurement_string);
    }

    fn perform_measurement_for_part(size: usize, path: &str) -> String {
        let number_of_instances = 30;
        let number_of_iterations = 1;

        let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();
        let mut reader = S6Reader::<SimpleGraph>::new(&file_result);

        let begin = time::Instant::now();

        let mut counter = 0;
        let mut all_true = true;
        while let Some(graph_result) = reader.next() {
            let mut graph = graph_result.unwrap();

            for number_of_iteration in 0..number_of_iterations {
                // let colourable = CvdDfsColourizer::is_colorable(&graph);
                let colourable = DFSColourizer::is_colorable(&graph);
                if colourable {
                    all_true = false;
                }
            }

            counter += 1;
        }

        let elapsed: f64 = (begin.elapsed().as_millis() as f64)
            / counter as f64
            / number_of_iterations as f64
            / 1000 as f64;
        format!("({}, {:.4})", size, elapsed)
    }

    // fn main() {
    //     // perform_measurements();
    //     perform_measurements_parallel();
    // }

    #[test]
    fn cvd_measurements() {
        perform_measurements();
    }
}
