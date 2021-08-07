use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::marker;

use serde::Serialize;

use crate::graph::undirected::UndirectedGraph;
use crate::procedure::helpers::config_helper;
use crate::procedure::procedure::{GraphProperties, Procedure, Result};
use crate::procedure::procedure_builder::{Config, ProcedureBuilder};

struct CounterProcedure<G: UndirectedGraph> {
    config: CounterProcedureConfig,
    _ph: marker::PhantomData<G>,
}

pub struct CounterProcedureConfig {
    print: bool,
}

pub struct CounterProcedureBuilder {}

impl<G: UndirectedGraph> Procedure<G> for CounterProcedure<G> {
    fn run(&self, graphs: &mut Vec<(G, GraphProperties)>) -> Result<()> {
        println!("running counter procedure");
        let mut props = HashMap::new();
        for graph in graphs {
            for property in graph.1.iter() {
                if property.0 == "graph-index" {
                    continue;
                }
                let property_hash = (property.0, property.1.to_string());
                if let Some(count) = props.get(&property_hash) {
                    props.insert(property_hash, count + 1);
                } else {
                    props.insert(property_hash, 1);
                }
            }
        }
        println!("count: ");
        for prop in props.iter() {
            println!("      {:?} : {}", prop.0, prop.1);
        }
        Ok(())
    }
}

impl CounterProcedureConfig {
    pub const PROC_TYPE: &'static str = "counter";

    pub fn from_proc_config(config: &HashMap<String, serde_json::Value>) -> Result<Self> {
        let print =
            config_helper::resolve_value_or_default(&config, "print", true, Self::PROC_TYPE)?;

        let result = CounterProcedureConfig { print };
        Ok(result)
    }

    pub fn print(&self) -> bool {
        self.print
    }
}

impl<G: UndirectedGraph + 'static> ProcedureBuilder<G> for CounterProcedureBuilder {
    fn build(&self, config: Config) -> Result<Box<dyn Procedure<G>>> {
        let proc_config = CounterProcedureConfig::from_proc_config(&config)?;
        Ok(Box::new(CounterProcedure {
            config: proc_config,
            _ph: marker::PhantomData,
        }))
    }
}
