use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    pub version: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,

    pub procedures: Vec<ProcedureConfig>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcedureConfig {
    #[serde(alias = "proc-type")]
    pub proc_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<String, String>>,
}

impl Configuration {
    pub fn from_yaml_string(yaml_string: &str) -> Result<Configuration, serde_yaml::Error> {
        serde_yaml::from_str(&yaml_string)
    }
}
