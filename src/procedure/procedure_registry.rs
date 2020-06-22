use crate::graph::graph::{Graph, GraphConstructor};
use crate::procedure::basic_procedures::colour::ColourProcedureBuilder;
use crate::procedure::basic_procedures::critical_and_stable_properties::CriticAndStablePropsProcedureBuilder;
use crate::procedure::basic_procedures::filter::FilterProcedureBuilder;
use crate::procedure::basic_procedures::read::ReadProcedureBuilder;
use crate::procedure::basic_procedures::unknown_procedure::UnknownProcedure;
use crate::procedure::basic_procedures::write::WriteProcedureBuilder;
use crate::procedure::configuration::ProcedureConfig;
use crate::procedure::procedure::Procedure;
use crate::procedure::procedure_builder::ProcedureBuilder;
use std::collections::HashMap;

pub struct ProcedureRegistry<G: Graph> {
    registry: HashMap<String, Box<dyn ProcedureBuilder<G>>>,
}

impl<G: Graph + GraphConstructor + Clone + 'static> ProcedureRegistry<G> {
    pub fn new() -> Self {
        ProcedureRegistry {
            registry: HashMap::new(),
        }
    }

    pub fn new_basic() -> Self {
        let mut reg = Self::new();
        reg.insert("read".to_string(), ReadProcedureBuilder {});
        reg.insert("write".to_string(), WriteProcedureBuilder {});
        reg.insert("colour".to_string(), ColourProcedureBuilder {});
        reg.insert("filter".to_string(), FilterProcedureBuilder {});
        reg.insert(
            "critical-and-stable".to_string(),
            CriticAndStablePropsProcedureBuilder {},
        );
        reg
    }

    pub fn insert<PB: 'static + ProcedureBuilder<G>>(
        &mut self,
        proc_type: String,
        proc_builder: PB,
    ) {
        self.registry.insert(proc_type, Box::new(proc_builder));
    }

    pub fn create_procedure(&self, config: ProcedureConfig) -> Box<dyn Procedure<G>> {
        let mut conf_map = HashMap::new();
        if config.config.is_some() {
            conf_map = config.config.unwrap();
        }

        for proc_type in self.registry.iter() {
            if config.proc_type.eq(proc_type.0) {
                let proc = proc_type.1.build(conf_map);
                return proc;
            }
        }

        // or just println right now
        Box::new(UnknownProcedure::of_type(config.proc_type))
    }
}
