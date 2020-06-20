use crate::error::Error;
use crate::graph::graph::Graph;
use crate::procedure::procedure::{BasicProperties, Procedure, Result, Config};
use crate::service::colour::bfs::BFSColourizer;
use crate::service::colour::colouriser::Colourizer;
use crate::service::colour::cvd_dfs::CvdDfsColourizer;
use crate::service::colour::sat::SATColourizer;
use std::marker;
use std::collections::HashMap;
use crate::procedure::procedure_builder::ProcedureBuilder;

struct ColourProcedure<G: Graph> {
    config: ColourProcedureConfig,
    _ph: marker::PhantomData<G>,
}

struct ColourProcedureConfig {
    config: HashMap<String, String>,
}

pub struct ColourProcedureBuilder {}

impl<G: Graph> Procedure<G> for ColourProcedure<G> {
    fn run(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()> {
        println!("running colour procedure");
        self.colour_graph(graphs)
    }
}

impl<G: Graph> ColourProcedure<G> {
    pub fn colour_graph(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()> {
        let colouriser_type_opt = self.config.colouriser_type()?;
        let colouriser_type;
        if colouriser_type_opt.is_none() {
            // resolve according to graph size

            colouriser_type = "bfs";
        } else {
            colouriser_type = colouriser_type_opt.unwrap();
        }

        match colouriser_type {
            "bfs" => {
                Self::color_by_colourizer::<BFSColourizer>(graphs);
            }
            "sat" => {
                Self::color_by_colourizer::<SATColourizer>(graphs);
            }
            "cvd-dfs" => {
                Self::color_by_colourizer::<CvdDfsColourizer>(graphs);
            }
            _ => {
                return Err(Error::ConfigError(String::from(
                    "unknown colouriser type for colour procedure",
                )));
            }
        }
        Ok(())
    }

    fn color_by_colourizer<C: Colourizer>(graphs: &mut Vec<(G, BasicProperties)>) {
        for graph in graphs {
            let result = C::is_colorable(&graph.0);
            graph
                .1
                .insert("colourable".to_string(), format!("{}", result));
        }
    }
}

impl ColourProcedureConfig {
    const PROC_TYPE: &'static str = "read";

    pub fn from_map(config: HashMap<String, String>) -> Self {
        ColourProcedureConfig { config }
    }

    pub fn colouriser_type(&self) -> Result<Option<&String>> {
        let colouriser_type_opt = self.config.get("colouriser-type");
        let colouriser_type;
        if colouriser_type_opt.is_none() {
            colouriser_type = None;
        } else {
            colouriser_type = Option::Some(colouriser_type_opt.unwrap());
        }
        Ok(colouriser_type)
    }
}

impl<G: Graph + 'static> ProcedureBuilder<G> for ColourProcedureBuilder {
    fn build(&self, config: Config) -> Box<dyn Procedure<G>> {
        Box::new(ColourProcedure {
            config: ColourProcedureConfig::from_map(config),
            _ph: marker::PhantomData,
        })
    }
}
