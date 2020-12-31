use serde::Serialize;
use std::collections::HashMap;

pub fn map_to_json_value<K: Serialize, V: Serialize>(
    map: &HashMap<K, V>,
) -> serde_json::Result<serde_json::Value> {
    let mut result: HashMap<String, serde_json::Value> = HashMap::new();
    for item in map.iter() {
        let key = serde_json::to_string(item.0)?;
        let value = serde_json::to_value(item.1)?;
        result.insert(key, value);
    }
    serde_json::to_value(result)
}

#[allow(dead_code)]
pub fn vec_to_json_value<T: Serialize>(vec: Vec<T>) -> serde_json::Result<serde_json::Value> {
    let mut result: Vec<(usize, T)> = Vec::with_capacity(vec.len());
    let mut i = 0;
    for item in vec {
        result.push((i, item));
        i += 1;
    }
    serde_json::to_value(result)
    // let mut result: HashMap<String, serde_json::Value> = HashMap::new();
    // let mut i = 0;
    // for item in vec.iter() {
    //     let key = serde_json::to_string(&i)?;
    //     let value = serde_json::to_value(item)?;
    //     result.insert(key, value);
    //     i += 1;
    // }
    // serde_json::to_value(result)
}
