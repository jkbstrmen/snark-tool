use crate::error::Error;
use crate::graph::graph::Graph;
use crate::procedure::config_helper;
use crate::procedure::procedure::{GraphProperties, Procedure, Result};
use crate::procedure::procedure_builder::{Config, ProcedureBuilder};
use crate::service::colour::bfs::BFSColourizer;
use crate::service::colour::colouriser::Colourizer;
use crate::service::colour::cvd_dfs::CvdDfsColourizer;
use crate::service::colour::sat::SATColourizer;
use std::collections::HashMap;
use std::marker;

struct ColourProcedure<G: Graph> {
    config: ColourProcedureConfig,
    _ph: marker::PhantomData<G>,
}

struct ColourProcedureConfig {
    colouriser_type: String,
}

pub struct ColourProcedureBuilder {}

impl<G: Graph> Procedure<G> for ColourProcedure<G> {
    fn run(&self, graphs: &mut Vec<(G, GraphProperties)>) -> Result<()> {
        println!("running colour procedure");
        self.colour_graph(graphs)
    }
}

impl<G: Graph> ColourProcedure<G> {
    pub fn colour_graph(&self, graphs: &mut Vec<(G, GraphProperties)>) -> Result<()> {
        let colouriser_type = self.config.colouriser_type();
        match colouriser_type.as_str() {
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

    fn color_by_colourizer<C: Colourizer>(graphs: &mut Vec<(G, GraphProperties)>) {
        for graph in graphs {
            let result = C::is_colorable(&graph.0);
            graph
                .1
                .insert("colourable".to_string(), serde_json::Value::Bool(result));
        }
    }
}

impl ColourProcedureConfig {
    const PROC_TYPE: &'static str = "colour";

    pub fn colouriser_type(&self) -> &String {
        &self.colouriser_type
    }

    pub fn from_proc_config(config: &HashMap<String, serde_json::Value>) -> Result<Self> {
        let colouriser_type = config_helper::resolve_value_or_default(
            &config,
            "colouriser-type",
            "bfs".to_string(),
            Self::PROC_TYPE,
        )?;
        let result = ColourProcedureConfig { colouriser_type };
        Ok(result)
    }
}

impl<G: Graph + 'static> ProcedureBuilder<G> for ColourProcedureBuilder {
    fn build(&self, config: Config) -> Result<Box<dyn Procedure<G>>> {
        let proc_config = ColourProcedureConfig::from_proc_config(&config)?;
        Ok(Box::new(ColourProcedure {
            config: proc_config,
            _ph: marker::PhantomData,
        }))
    }
}
