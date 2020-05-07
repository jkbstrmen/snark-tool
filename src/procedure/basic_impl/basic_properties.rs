use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicProperties {
    pub colorable: Option<bool>,
}

impl BasicProperties {
    pub fn new() -> Self {
        BasicProperties { colorable: None }
    }
}
