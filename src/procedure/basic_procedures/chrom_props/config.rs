use crate::procedure::basic_procedures::colour::ColouriserType;
use crate::procedure::error::Error;
use crate::procedure::helpers::config_helper;
use crate::procedure::procedure::Result;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

pub const DFS_COLOURISER: &str = "dfs";
pub const SAT_COLOURISER: &str = "sat";

// property
pub const CRITICAL: &str = "critical";
pub const COCRITICAL: &str = "cocritical";
pub const VERTEX_SUBCRITICAL: &str = "vertex-subcritical";
pub const EDGE_SUBCRITICAL: &str = "edge-subcritical";
pub const ACRITICAL: &str = "acritical";
pub const STABLE: &str = "stable";
pub const COSTABLE: &str = "costable";
pub const RESISTANCE: &str = "resistance";
pub const GIRTH: &str = "girth";
pub const CYCLIC_EDGE_CONNECTIVITY: &str = "cyclic-edge-connectivity";
pub const EDGE_RESISTIBILITY: &str = "edge-resistibility";
pub const VERTEX_RESISTIBILITY: &str = "vertex-resistibility";
pub const ODDNESS: &str = "oddness";

pub const VERTEX_RESISTIBILITIES: &str = "vertex-resistibilities";
pub const VERTEX_RESISTIBILITY_INDEX: &str = "vertex-resistibility-index";
pub const EDGE_RESISTIBILITIES: &str = "edge-resistibilities";
pub const EDGE_RESISTIBILITY_INDEX: &str = "edge-resistibility-index";

// property name
pub const COLOURISER_TYPE: &str = "colouriser-type";
pub const PARALLEL: &str = "parallel";
pub const PARALLELIZATION: &str = "parallelization";
pub const PROPERTIES: &str = "properties";
pub const GRAPH_INDEX: &str = "graph-index";
pub const MAX_THREADS: &str = "max-threads";

pub struct ChromaticPropsProcedureConfig {
    pub colouriser_type: ColouriserType,
    // TODO - change from bool to enum - chunk-based (for plenty of smaller graphs), graph-based (for bigger graphs)
    // pub parallel: bool,
    pub parallelization: ParallelizationType,
    pub properties_to_compute: ChromaticPropertiesToCompute,
    pub max_threads: usize,
}

impl ChromaticPropsProcedureConfig {
    pub const PROC_TYPE: &'static str = "chromatic-properties";

    pub fn from_proc_config(config: &HashMap<String, serde_json::Value>) -> Result<Self> {
        let colouriser_type = config_helper::resolve_value_or_default(
            &config,
            COLOURISER_TYPE,
            DFS_COLOURISER.to_string(),
            Self::PROC_TYPE,
        )?;
        let parallel =
            config_helper::resolve_value_or_default(&config, PARALLEL, true, Self::PROC_TYPE)?;

        let parallelization_str = config_helper::resolve_value_or_default(
            &config,
            PARALLELIZATION,
            PARALL_NONE.to_string(),
            Self::PROC_TYPE,
        )?;

        let properties = config_helper::resolve_value(&config, PROPERTIES, Self::PROC_TYPE)?;
        let properties_to_compute = ChromaticPropertiesToCompute::new();

        let cpus_count = num_cpus::get();
        let max_cpus = config_helper::resolve_value_or_default(
            &config,
            MAX_THREADS,
            cpus_count,
            Self::PROC_TYPE,
        )?;

        let mut result = ChromaticPropsProcedureConfig {
            colouriser_type: ColouriserType::from_string(&colouriser_type)?,
            // parallel,
            parallelization: ParallelizationType::from_string(parallelization_str)?,
            properties_to_compute,
            max_threads: max_cpus
        };
        result.resolve_properties_to_compute(properties);
        Ok(result)
    }

    fn resolve_properties_to_compute(&mut self, properties: Vec<String>) {
        for property in properties.iter() {
            match property.as_str() {
                CRITICAL => {
                    self.properties_to_compute.critical = true;
                }
                COCRITICAL => {
                    self.properties_to_compute.cocritical = true;
                }
                VERTEX_SUBCRITICAL => {
                    self.properties_to_compute.vertex_subcritical = true;
                }
                EDGE_SUBCRITICAL => {
                    self.properties_to_compute.edge_subcritical = true;
                }
                ACRITICAL => {
                    self.properties_to_compute.acritical = true;
                }
                STABLE => {
                    self.properties_to_compute.stable = true;
                }
                COSTABLE => {
                    self.properties_to_compute.costable = true;
                }
                GIRTH => {
                    self.properties_to_compute.girth = true;
                }
                CYCLIC_EDGE_CONNECTIVITY => {
                    self.properties_to_compute.cyclic_connectivity = true;
                }
                RESISTANCE => {
                    self.properties_to_compute.resistance = true;
                }
                EDGE_RESISTIBILITY => {
                    self.properties_to_compute.edge_resistibility = true;
                }
                VERTEX_RESISTIBILITY => {
                    self.properties_to_compute.vertex_resistibility = true;
                }
                ODDNESS => {
                    self.properties_to_compute.oddness = true;
                }
                _ => {}
            }
        }
    }

    pub fn colouriser_type(&self) -> &ColouriserType {
        &self.colouriser_type
    }

    pub fn parallelization(&self) -> &ParallelizationType {
        &self.parallelization
    }
}

#[derive(Clone)]
pub struct ChromaticPropertiesToCompute {
    pub critical: bool,
    pub cocritical: bool,
    pub vertex_subcritical: bool,
    pub edge_subcritical: bool,
    pub acritical: bool,
    pub stable: bool,
    pub costable: bool,

    pub resistance: bool,
    pub edge_resistibility: bool,
    pub vertex_resistibility: bool,
    pub girth: bool,
    pub cyclic_connectivity: bool,
    pub oddness: bool,
}

impl ChromaticPropertiesToCompute {
    pub fn new() -> Self {
        ChromaticPropertiesToCompute {
            critical: false,
            cocritical: false,
            vertex_subcritical: false,
            edge_subcritical: false,
            acritical: false,
            stable: false,
            costable: false,
            resistance: false,
            edge_resistibility: false,
            vertex_resistibility: false,
            girth: false,
            cyclic_connectivity: false,
            oddness: false,
        }
    }
}

// parallelization types
const PARALL_BATCH: &str = "batch-based";
const PARALL_GRAPH: &str = "graph-based";
const PARALL_NONE: &str = "none";

#[derive(Clone, Deserialize)]
pub enum ParallelizationType {
    BatchBased,
    GraphBased,
    None,
}

impl ParallelizationType {
    pub fn from_string(string: impl AsRef<str>) -> Result<ParallelizationType> {
        let mut parallel_type = ParallelizationType::None;
        match string.as_ref() {
            PARALL_BATCH => parallel_type = ParallelizationType::BatchBased,
            PARALL_GRAPH => parallel_type = ParallelizationType::GraphBased,
            PARALL_NONE => parallel_type = ParallelizationType::None,
            _ => {
                return Err(Error::ConfigError(String::from(format!(
                    "unknown parallelization type: {}, did you mean {}, {} or {}?",
                    string.as_ref(),
                    PARALL_BATCH,
                    PARALL_GRAPH,
                    PARALL_NONE
                ))));
            }
        }
        Ok(parallel_type)
    }
}
