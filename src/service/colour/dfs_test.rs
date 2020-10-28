use crate::graph::graph::Graph;
use crate::service::colour::colouriser::Colourizer;
use crate::service::colour::cvd;
use crate::service::colour::cvd::{ALL_CALLS, DFS_AFTER_CVD_WHEN_TRUE, ELAPSED, COUNTER_3, ELAPSED_0, DFS_CALLS, ELAPSED_2, ELAPSED_3};
use crate::service::colour::dfs_improved::DFSColourizer;
use std::time;

// Colorizer for (sub)cubic graphs only
pub struct DfsDfsColourizer {}

impl Colourizer for DfsDfsColourizer {
    fn is_colorable<G>(graph: &G) -> bool
        where
            G: Graph,
    {
        unsafe {
            ALL_CALLS += 1;
        }

        let begin = time::Instant::now();

        let result = DFSColourizer::is_colorable(graph);

        unsafe {
            ELAPSED_0 += begin.elapsed().as_nanos();
        }

        if result {
            unsafe {
                ELAPSED_2 += begin.elapsed().as_nanos();
                COUNTER_3 += 1;
            }
        } else {
            unsafe {
                ELAPSED_3 += begin.elapsed().as_nanos();
            }
        }



        result
    }

    fn new() -> Self {
        DfsDfsColourizer {}
    }
}
