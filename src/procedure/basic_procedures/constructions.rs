use crate::graph::graph::Graph;
use crate::procedure::helpers::config_helper;
use crate::procedure::procedure;
use crate::procedure::procedure::{GraphProperties, Procedure};
use crate::procedure::procedure_builder::{Config, ProcedureBuilder};
use crate::service::colour::colouriser::Colouriser;
use crate::service::colour::dfs_improved::DFSColourizer;
use crate::service::constructions::error::ConstructionError;
use crate::service::constructions::i_extension::{i_extension, IExtensions};
use std::collections::HashMap;
use std::str::FromStr;
use std::{marker, result};

pub type Result<T> = result::Result<T, ConstructionError>;

// config fields
const CONSTRUCTION_TYPE: &str = "construction-type";

#[derive(Debug)]
pub enum ConstructionType {
    DotProduct,
    IExtension,
    YExtension,
}

impl FromStr for ConstructionType {
    type Err = ConstructionError;

    fn from_str(input: &str) -> Result<Self> {
        match input {
            "dot-product" => Ok(ConstructionType::DotProduct),
            "i-extension" => Ok(ConstructionType::IExtension),
            "y-extension" => Ok(ConstructionType::YExtension),
            _ => Err(ConstructionError::new(format!(
                "unknown construction type: {}",
                input
            ))),
        }
    }
}

struct ConstructionProcedure<G> {
    config: ConstructionProcedureConfig,
    _ph: marker::PhantomData<G>,
}

pub struct ConstructionProcedureConfig {
    construction_type: ConstructionType,
}

pub struct ConstructionProcedureBuilder {}

impl<G: Graph> Procedure<G> for ConstructionProcedure<G> {
    fn run(&self, graphs: &mut Vec<(G, GraphProperties)>) -> procedure::Result<()> {
        println!(
            "running {} procedure",
            ConstructionProcedureConfig::PROC_TYPE
        );
        self.construct(graphs)?;
        Ok(())
    }
}

impl<G: Graph> ConstructionProcedure<G> {
    pub fn construct(&self, graphs: &mut Vec<(G, GraphProperties)>) -> Result<()> {
        for graph in graphs.iter() {
            match self.config.construction_type {
                ConstructionType::DotProduct => {}
                ConstructionType::IExtension => {
                    // let colouriser = DFSColourizer::new();
                    // let mut i_extensions = IExtensions::new(graph, &colouriser);
                    // let extended = i_extensions.next().unwrap();
                    // graphs.push(extended);
                }
                ConstructionType::YExtension => {}
            }
        }
        Ok(())
    }
}

impl ConstructionProcedureConfig {
    pub const PROC_TYPE: &'static str = "construction";

    pub fn from_proc_config(config: &HashMap<String, serde_json::Value>) -> Result<Self> {
        let construction_type_string: String =
            config_helper::resolve_value(&config, CONSTRUCTION_TYPE, Self::PROC_TYPE)?;
        let construction_type = ConstructionType::from_str(&construction_type_string)?;
        let result = ConstructionProcedureConfig { construction_type };
        Ok(result)
    }

    pub fn construction_type(&self) -> &ConstructionType {
        &self.construction_type
    }
}

impl<G: Graph + 'static> ProcedureBuilder<G> for ConstructionProcedureBuilder {
    fn build(&self, config: Config) -> procedure::Result<Box<dyn Procedure<G>>> {
        let proc_config = ConstructionProcedureConfig::from_proc_config(&config)?;
        Ok(Box::new(ConstructionProcedure {
            config: proc_config,
            _ph: marker::PhantomData,
        }))
    }
}
