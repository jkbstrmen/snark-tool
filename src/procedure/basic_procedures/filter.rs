use crate::graph::graph::Graph;
use crate::procedure::helpers::config_helper;
use crate::procedure::procedure::{GraphProperties, Procedure, Result};
use crate::procedure::procedure_builder::{Config, ProcedureBuilder};
use std::collections::HashMap;
use std::marker;

struct FilterProcedure<G> {
    config: FilterProcedureConfig,
    _ph: marker::PhantomData<G>,
}

pub struct FilterProcedureConfig {
    filter_by: GraphProperties,
}

pub struct FilterProcedureBuilder {}

impl<G: Graph> Procedure<G> for FilterProcedure<G> {
    fn run(&self, graphs: &mut Vec<(G, GraphProperties)>) -> Result<()> {
        println!("running filter procedure");
        self.filter(graphs)
    }
}

impl<G: Graph> FilterProcedure<G> {
    pub fn filter(&self, graphs: &mut Vec<(G, GraphProperties)>) -> Result<()> {
        let filter_properties = self.config.filter_by();
        graphs.retain(|graph| {
            let mut retain = true;
            for filter_property in filter_properties {
                let mut has_property = false;
                for graph_property in &graph.1 {
                    if filter_property == graph_property {
                        has_property = true;
                    }
                }
                if !has_property {
                    retain = false;
                    break;
                }
            }
            retain
        });
        Ok(())
    }
}

impl FilterProcedureConfig {
    pub const PROC_TYPE: &'static str = "filter";

    pub fn from_proc_config(config: &HashMap<String, serde_json::Value>) -> Result<Self> {
        let filter_by = config_helper::resolve_value(&config, "filter-by", Self::PROC_TYPE)?;

        let result = FilterProcedureConfig { filter_by };
        Ok(result)
    }

    pub fn filter_by(&self) -> &GraphProperties {
        &self.filter_by
    }
}

impl<G: Graph + 'static> ProcedureBuilder<G> for FilterProcedureBuilder {
    fn build(&self, config: Config) -> Result<Box<dyn Procedure<G>>> {
        let proc_config = FilterProcedureConfig::from_proc_config(&config)?;
        Ok(Box::new(FilterProcedure {
            config: proc_config,
            _ph: marker::PhantomData,
        }))
    }
}
