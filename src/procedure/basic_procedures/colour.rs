use crate::graph::undirected::UndirectedGraph;
use crate::procedure::error::Error;
use crate::procedure::helpers::config_helper;
use crate::procedure::procedure::{GraphProperties, Procedure, Result};
use crate::procedure::procedure_builder::{Config, ProcedureBuilder};
use crate::service::colour::colouriser::Colouriser;
use crate::service::colour::cvd::cvd;
use crate::service::colour::cvd::cvd_dfs::CvdDfsColourizer;
use crate::service::colour::cvd::cvd_sat::CvdSatColourizer;
use crate::service::colour::matchings::matching_col::MatchingColouriser;
use crate::service::colour::recursive::dfs_improved::DFSColourizer;
use crate::service::colour::sat::sat::SATColourizer;
use std::collections::HashMap;
use std::marker;

// coloriser types
const DFS: &str = "dfs";
const SAT: &str = "sat";
const CVD: &str = "cvd";
const CVD_DFS: &str = "cvd-dfs";
const CVD_SAT: &str = "cvd-sat";
const MATCHING: &str = "matching";
const AUTO: &str = "auto";

#[derive(Clone)]
pub enum ColouriserType {
    Dfs,
    Sat,
    Cvd,
    CvdDfs,
    CvdSat,
    Matching,
    Auto,
}

impl ColouriserType {
    pub fn from_string(string: &String) -> Result<ColouriserType> {
        let col_type;
        match string.as_str() {
            DFS => col_type = ColouriserType::Dfs,
            SAT => col_type = ColouriserType::Sat,
            CVD => col_type = ColouriserType::Cvd,
            CVD_DFS => col_type = ColouriserType::CvdDfs,
            CVD_SAT => col_type = ColouriserType::CvdSat,
            MATCHING => col_type = ColouriserType::Matching,
            AUTO => col_type = ColouriserType::Auto,
            &_ => {
                return Err(Error::ConfigError(String::from(format!(
                    "unknown colouriser type: {}, did you mean {}, {}, {}, {}, {}, {} or {}?",
                    string, DFS, SAT, MATCHING, CVD, CVD_DFS, CVD_SAT, AUTO
                ))));
            }
        }
        Ok(col_type)
    }
}

struct ColourProcedure<G: UndirectedGraph> {
    config: ColourProcedureConfig,
    _ph: marker::PhantomData<G>,
}

pub struct ColourProcedureConfig {
    colouriser_type: ColouriserType,
}

pub struct ColourProcedureBuilder {}

impl<G: UndirectedGraph> Procedure<G> for ColourProcedure<G> {
    fn run(&self, graphs: &mut Vec<(G, GraphProperties)>) -> Result<()> {
        println!("running colour procedure");
        self.colour_graph(graphs)
    }
}

impl<G: UndirectedGraph> ColourProcedure<G> {
    pub fn colour_graph(&self, graphs: &mut Vec<(G, GraphProperties)>) -> Result<()> {
        let colouriser_type = self.config.colouriser_type();
        match colouriser_type {
            ColouriserType::Dfs => {
                Self::color_by_colourizer::<DFSColourizer>(graphs);
            }
            ColouriserType::Sat => {
                Self::color_by_colourizer::<SATColourizer>(graphs);
            }
            ColouriserType::CvdDfs => {
                Self::color_by_colourizer::<CvdDfsColourizer>(graphs);
            }
            ColouriserType::CvdSat => {
                Self::color_by_colourizer::<CvdSatColourizer>(graphs);
            }
            ColouriserType::Matching => {
                Self::color_by_colourizer::<MatchingColouriser>(graphs);
            }
            ColouriserType::Cvd => {
                Self::color_by_colourizer_cvd(graphs);
            }
            _ => {
                return Err(Error::ConfigError(String::from(
                    "unknown colouriser type for colour procedure",
                )));
            }
        }
        Ok(())
    }

    fn color_by_colourizer<C: Colouriser>(graphs: &mut Vec<(G, GraphProperties)>) {
        for graph in graphs {
            let result = C::is_colorable(&graph.0);
            graph
                .1
                .insert("colourable".to_string(), serde_json::Value::Bool(result));
        }
    }

    ///
    /// color by heuristic
    ///
    fn color_by_colourizer_cvd(graphs: &mut Vec<(G, GraphProperties)>) {
        for graph in graphs {
            let result_option = cvd::is_colorable(&graph.0);
            if let Some(result) = result_option {
                graph
                    .1
                    .insert("colourable".to_string(), serde_json::Value::Bool(result));
            }
        }
    }
}

impl ColourProcedureConfig {
    pub const PROC_TYPE: &'static str = "colour";

    pub fn colouriser_type(&self) -> &ColouriserType {
        &self.colouriser_type
    }

    pub fn from_proc_config(config: &HashMap<String, serde_json::Value>) -> Result<Self> {
        let colouriser_type = config_helper::resolve_value_or_default(
            &config,
            "colouriser-type",
            DFS.to_string(),
            Self::PROC_TYPE,
        )?;

        let result = ColourProcedureConfig {
            colouriser_type: ColouriserType::from_string(&colouriser_type)?,
        };
        Ok(result)
    }
}

impl<G: UndirectedGraph + 'static> ProcedureBuilder<G> for ColourProcedureBuilder {
    fn build(&self, config: Config) -> Result<Box<dyn Procedure<G>>> {
        let proc_config = ColourProcedureConfig::from_proc_config(&config)?;
        Ok(Box::new(ColourProcedure {
            config: proc_config,
            _ph: marker::PhantomData,
        }))
    }
}
