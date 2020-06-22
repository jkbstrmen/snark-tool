use crate::graph::graph::Graph;
use crate::procedure::procedure::{BasicProperties, Config, Procedure, Result};
use crate::procedure::procedure_builder::ProcedureBuilder;
use std::collections::HashMap;
use std::marker;

struct FilterProcedure<G> {
    config: FilterProcedureConfig,
    _ph: marker::PhantomData<G>,
}

struct FilterProcedureConfig {
    config: HashMap<String, String>,
}

pub struct FilterProcedureBuilder {}

impl<G: Graph> Procedure<G> for FilterProcedure<G> {
    fn run(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()> {
        println!("running filter procedure");
        self.filter(graphs)
    }
}

impl<G: Graph> FilterProcedure<G> {
    pub fn filter(&self, graphs: &mut Vec<(G, BasicProperties)>) -> Result<()> {
        let filter_properties = self.config.filter_properties();
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
    const PROC_TYPE: &'static str = "read";

    pub fn from_map(config: HashMap<String, String>) -> Self {
        Self { config }
    }

    pub fn filter_properties(&self) -> &HashMap<String, String> {
        &self.config
    }
}

impl<G: Graph + 'static> ProcedureBuilder<G> for FilterProcedureBuilder {
    fn build(&self, config: Config) -> Box<dyn Procedure<G>> {
        Box::new(FilterProcedure {
            config: FilterProcedureConfig::from_map(config),
            _ph: marker::PhantomData,
        })
    }
}
