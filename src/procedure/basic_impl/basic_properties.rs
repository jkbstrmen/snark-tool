use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicProperties {
    pub colorable: bool,
}

impl BasicProperties {
    pub fn new() -> Self {
        BasicProperties { colorable: false }
    }
}
