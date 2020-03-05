use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Configuration{

    pub version: f64,
    pub options: Vec<String>,
    pub procedures: HashMap<String, HashMap<String, String>>

}

// pub struct ProceduresConfig{
//
//     read: HashMap<String, String>,
// }
