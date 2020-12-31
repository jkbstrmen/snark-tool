use crate::graph::graph::GraphConstructor;
use crate::graph::undirected::simple_graph::graph::SimpleGraph;
use crate::graph::undirected::UndirectedGraph;
use crate::procedure::helpers::config_helper;
use crate::procedure::procedure;
use crate::procedure::procedure::{GraphProperties, Procedure};
use crate::procedure::procedure_builder::{Config, ProcedureBuilder};
use crate::service::colour::colouriser::Colouriser;
use crate::service::colour::dfs_improved::DFSColourizer;
use crate::service::constructions::dot_product::DotProducts;
use crate::service::constructions::error::ConstructionError;
use crate::service::constructions::i_extension::IExtensions;
use crate::service::constructions::y_extension::YExtensions;
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

impl<G: UndirectedGraph> Procedure<G> for ConstructionProcedure<G> {
    fn run(&self, graphs: &mut Vec<(G, GraphProperties)>) -> procedure::Result<()> {
        println!(
            "running {} procedure",
            ConstructionProcedureConfig::PROC_TYPE
        );
        self.construct(graphs)?;
        Ok(())
    }
}

impl<G: UndirectedGraph> ConstructionProcedure<G> {
    pub fn construct(&self, _graphs: &mut Vec<(G, GraphProperties)>) -> Result<()> {
        // TODO - finish when graphs are Undirected

        if ConstructionProcedureConfig::PROC_TYPE == "construction" {
            println!("Constructions are not ready yet.");
            return Ok(());
        }

        for _graph in _graphs.iter() {
            match self.config.construction_type {
                ConstructionType::DotProduct => {
                    let graph = SimpleGraph::new();
                    let mut dot_products = DotProducts::new(&graph, &graph);
                    let _extended = dot_products.next().unwrap();
                    // graphs.push(extended);
                }
                ConstructionType::IExtension => {
                    let graph = SimpleGraph::new();
                    let colouriser = DFSColourizer::new();
                    let mut i_extensions = IExtensions::new(&graph, &colouriser);
                    let _extended = i_extensions.next().unwrap();
                    // graphs.push(extended);
                }
                ConstructionType::YExtension => {
                    let graph = SimpleGraph::new();
                    let colouriser = DFSColourizer::new();
                    let mut y_extensions = YExtensions::new(&graph, &colouriser);
                    let _extended = y_extensions.next().unwrap();
                    // graphs.push(extended);
                }
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

    // pub fn construction_type(&self) -> &ConstructionType {
    //     &self.construction_type
    // }
}

impl<G: UndirectedGraph + 'static> ProcedureBuilder<G> for ConstructionProcedureBuilder {
    fn build(&self, config: Config) -> procedure::Result<Box<dyn Procedure<G>>> {
        let proc_config = ConstructionProcedureConfig::from_proc_config(&config)?;
        Ok(Box::new(ConstructionProcedure {
            config: proc_config,
            _ph: marker::PhantomData,
        }))
    }
}
