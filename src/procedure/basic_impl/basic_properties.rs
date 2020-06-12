use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicProperties {
    pub properties: HashMap<String, String>,
}

impl BasicProperties {
    pub fn new() -> Self {
        BasicProperties {
            properties: HashMap::new(),
        }
    }
}
