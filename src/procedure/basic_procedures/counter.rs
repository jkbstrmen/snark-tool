use std::collections::HashMap;
use std::marker;

use crate::graph::undirected::UndirectedGraph;
use crate::procedure::helpers::config_helper;
use crate::procedure::procedure::{GraphProperties, Procedure, Result};
use crate::procedure::procedure_builder::{ConfigMap, ProcedureBuilder};

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

                // like this because borrow checker
                let count_opt = props.get(&property_hash);
                let mut count_opt_copy: Option<usize> = None;
                if let Some(count) = count_opt {
                    count_opt_copy = Some(*count);
                }

                // if let Some(count) = props.get(&property_hash) {
                if let Some(count) = count_opt_copy {
                    props.insert(property_hash, count + 1);
                } else {
                    props.insert(property_hash, 1);
                }
            }
        }
        if self.config.print() {
            println!("count: ");
            for prop in props.iter() {
                println!("      {:?} : {}", prop.0, prop.1);
            }
        }
        Ok(())
    }
}

impl CounterProcedureConfig {
    pub const PROC_TYPE: &'static str = "count";

    pub fn new(print: bool) -> Self {
        CounterProcedureConfig { print }
    }

    pub fn default() -> Self {
        CounterProcedureConfig { print: true }
    }

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
    fn build_from_map(&self, config: ConfigMap) -> Result<Box<dyn Procedure<G>>> {
        let proc_config = CounterProcedureConfig::from_proc_config(&config)?;
        Ok(Box::new(CounterProcedure {
            config: proc_config,
            _ph: marker::PhantomData,
        }))
    }
}

impl CounterProcedureBuilder {
    pub fn build<G: UndirectedGraph + 'static>(
        config: CounterProcedureConfig,
    ) -> Box<dyn Procedure<G>> {
        Box::new(CounterProcedure {
            config,
            _ph: marker::PhantomData,
        })
    }
}
