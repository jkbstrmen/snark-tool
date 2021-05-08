use crate::graph::undirected::UndirectedGraph;
use crate::procedure::error::Error;
use crate::procedure::helpers::config_helper;
use crate::procedure::procedure::{GraphProperties, Procedure, Result};
use crate::procedure::procedure_builder::{Config, ProcedureBuilder};
use serde_json::Value;
use std::collections::HashMap;
use std::{marker, result};

// keys
const COMPARATOR: &str = "comparator";
const VALUE: &str = "value";

// comparators
const GREATER_THAN: &str = ">";
const LESS_THAN: &str = "<";
const EQUALS: &str = "=";
const NOT_EQUAL: &str = "!=";
const GREATER_THAN_OR_EQUAL_TO: &str = ">=";
const LESS_THAN_OR_EQUAL_TO: &str = "<=";

struct FilterProcedure<G> {
    config: FilterProcedureConfig,
    _ph: marker::PhantomData<G>,
}

pub struct FilterProcedureConfig {
    filter_by: GraphProperties,
}

pub struct FilterProcedureBuilder {}

impl<G: UndirectedGraph> Procedure<G> for FilterProcedure<G> {
    fn run(&self, graphs: &mut Vec<(G, GraphProperties)>) -> Result<()> {
        println!("running filter procedure");
        self.filter(graphs)
    }
}

impl<G: UndirectedGraph> FilterProcedure<G> {
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
                    if has_property == false && filter_property.0 == graph_property.0 {
                        let result = compare_values(filter_property, graph_property);
                        if result.is_err() {
                            // TODO - return result
                            eprintln!("malformed filter property: {}", result.err().unwrap());
                            return false;
                        }
                        has_property = result.unwrap();
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

fn compare_values(
    filter_property: (&String, &Value),
    graph_property: (&String, &Value),
) -> Result<bool> {
    let mut conditions_met = false;

    let field_value_result: result::Result<HashMap<String, serde_json::Value>, serde_json::Error> =
        serde_json::from_value(filter_property.1.clone());
    let field_value = field_value_result?;
    let comparator: String =
        config_helper::resolve_value(&field_value, COMPARATOR, FilterProcedureConfig::PROC_TYPE)?;
    let filter_value: u64 =
        config_helper::resolve_value(&field_value, VALUE, FilterProcedureConfig::PROC_TYPE)?;

    let graph_property_value: u64 = serde_json::from_value(graph_property.1.clone())?;

    match comparator.as_str() {
        GREATER_THAN => {
            if graph_property_value > filter_value {
                conditions_met = true;
            }
        }
        LESS_THAN => {
            if graph_property_value < filter_value {
                conditions_met = true;
            }
        }
        EQUALS => {
            if graph_property_value == filter_value {
                conditions_met = true;
            }
        }
        NOT_EQUAL => {
            if graph_property_value != filter_value {
                conditions_met = true;
            }
        }
        GREATER_THAN_OR_EQUAL_TO => {
            if graph_property_value >= filter_value {
                conditions_met = true;
            }
        }
        LESS_THAN_OR_EQUAL_TO => {
            if graph_property_value <= filter_value {
                conditions_met = true;
            }
        }
        _ => {
            return Err(Error::ConfigError(format!(
                "not supported comparator '{}' for filter property '{}'",
                comparator, filter_property.0
            )));
        }
    }
    Ok(conditions_met)
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

impl<G: UndirectedGraph + 'static> ProcedureBuilder<G> for FilterProcedureBuilder {
    fn build(&self, config: Config) -> Result<Box<dyn Procedure<G>>> {
        let proc_config = FilterProcedureConfig::from_proc_config(&config)?;
        Ok(Box::new(FilterProcedure {
            config: proc_config,
            _ph: marker::PhantomData,
        }))
    }
}
