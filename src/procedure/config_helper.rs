use crate::error::Error;
use crate::procedure::procedure::Result;
use std::collections::HashMap;
use std::result;

pub fn resolve_value_or_default<V: serde::de::DeserializeOwned, T: AsRef<str>>(
    config: &HashMap<String, serde_json::Value>,
    field: T,
    default_value: V,
    procedure: T,
) -> Result<V> {
    let field_value_opt = config.get(field.as_ref());
    let field_value;
    if field_value_opt.is_none() {
        field_value = default_value;
    } else {
        let val = field_value_opt.unwrap();
        let field_value_result: result::Result<V, serde_json::Error> =
            serde_json::from_value(val.clone());
        if field_value_result.is_err() {
            return Err(Error::ConfigError(format!(
                "field: {} for procedure: {} is in wrong format: {}",
                field.as_ref(),
                procedure.as_ref(),
                field_value_result.err().unwrap()
            )));
        }
        field_value = field_value_result.unwrap();
    }
    Ok(field_value)
}

pub fn resolve_value<V: serde::de::DeserializeOwned, T: AsRef<str>>(
    config: &HashMap<String, serde_json::Value>,
    field: T,
    procedure: T,
) -> Result<V> {
    let field_value_opt = config.get(field.as_ref());
    let field_value;
    if field_value_opt.is_none() {
        return Err(Error::ConfigError(format!(
            "field: {} for procedure: {} is missing",
            field.as_ref(),
            procedure.as_ref()
        )));
    } else {
        let val = field_value_opt.unwrap();
        let field_value_result: result::Result<V, serde_json::Error> =
            serde_json::from_value(val.clone());
        if field_value_result.is_err() {
            return Err(Error::ConfigError(format!(
                "field: {} for procedure: {} is in wrong format: {}",
                field.as_ref(),
                procedure.as_ref(),
                field_value_result.err().unwrap()
            )));
        }
        field_value = field_value_result.unwrap();
    }
    Ok(field_value)
}
