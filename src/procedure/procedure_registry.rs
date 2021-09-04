use std::collections::HashMap;

use crate::graph::graph::GraphConstructor;
use crate::graph::undirected::UndirectedGraph;
use crate::procedure::basic_procedures::chrom_props::chromatic_properties::ChromaticPropsProcedureBuilder;
use crate::procedure::basic_procedures::chrom_props::config::ChromaticPropsProcedureConfig;
use crate::procedure::basic_procedures::colour::{ColourProcedureBuilder, ColourProcedureConfig};
use crate::procedure::basic_procedures::constructions::{
    ConstructionProcedureBuilder, ConstructionProcedureConfig,
};
use crate::procedure::basic_procedures::counter::{
    CounterProcedureBuilder, CounterProcedureConfig,
};
use crate::procedure::basic_procedures::filter::{FilterProcedureBuilder, FilterProcedureConfig};
use crate::procedure::basic_procedures::read::{ReadProcedureBuilder, ReadProcedureConfig};
use crate::procedure::basic_procedures::unknown_procedure::UnknownProcedure;
use crate::procedure::basic_procedures::write::{WriteProcedureBuilder, WriteProcedureConfig};
use crate::procedure::configuration::ProcedureConfig;
use crate::procedure::procedure::{Procedure, Result};
use crate::procedure::procedure_builder::ProcedureBuilder;

pub struct ProcedureRegistry<G: UndirectedGraph> {
    registry: HashMap<String, Box<dyn ProcedureBuilder<G>>>,
}

impl<G: UndirectedGraph + GraphConstructor + Clone + 'static> ProcedureRegistry<G> {
    pub fn new() -> Self {
        ProcedureRegistry {
            registry: HashMap::new(),
        }
    }

    pub fn new_basic() -> Self {
        let mut reg = Self::new();
        reg.insert(
            ReadProcedureConfig::PROC_TYPE.to_string(),
            ReadProcedureBuilder {},
        );
        reg.insert(
            WriteProcedureConfig::PROC_TYPE.to_string(),
            WriteProcedureBuilder {},
        );
        reg.insert(
            ColourProcedureConfig::PROC_TYPE.to_string(),
            ColourProcedureBuilder {},
        );
        reg.insert(
            FilterProcedureConfig::PROC_TYPE.to_string(),
            FilterProcedureBuilder {},
        );
        reg.insert(
            ChromaticPropsProcedureConfig::PROC_TYPE.to_string(),
            ChromaticPropsProcedureBuilder {},
        );
        reg.insert(
            ConstructionProcedureConfig::PROC_TYPE.to_string(),
            ConstructionProcedureBuilder {},
        );
        reg.insert(
            CounterProcedureConfig::PROC_TYPE.to_string(),
            CounterProcedureBuilder {},
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

    pub fn create_procedure(&self, config: ProcedureConfig) -> Result<Box<dyn Procedure<G>>> {
        let mut conf_map = HashMap::new();
        if config.config.is_some() {
            conf_map = config.config.unwrap();
        }

        let proc_builder = self.registry.get(&config.proc_type);
        if let Some(builder) = proc_builder {
            let proc = builder.build_from_map(conf_map);
            return proc;
        }

        // or just println right now
        Ok(Box::new(UnknownProcedure::of_type(config.proc_type)))
    }
}
