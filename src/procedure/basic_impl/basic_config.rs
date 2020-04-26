use crate::error::Error;
use crate::procedure::procedure::Config;
use std::result;
use std::str::FromStr;

type Result<T> = result::Result<T, Error>;

pub struct BasicConfig {
    config: Config,
    proc_type: String,
}

impl BasicConfig {
    pub fn from_config(config: Config, proc_type: String) -> Self {
        BasicConfig { config, proc_type }
    }

    pub fn get_file(&self) -> Result<&String> {
        let file_path_opt = self.config.get("file");
        if file_path_opt.is_none() {
            return Err(Error::ConfigError(format!(
                "file not specified for procedure: {}",
                self.proc_type
            )));
        }
        Ok(file_path_opt.unwrap())
    }

    pub fn get_graph_format(&self) -> Result<&String> {
        let graph_format = self.config.get("graph-format");
        if graph_format.is_none() {
            return Err(Error::ConfigError(format!(
                "missing graph format for procedure: {}",
                self.proc_type
            )));
        }
        Ok(graph_format.unwrap())
    }

    pub fn get_number_of_graphs(&self) -> Result<Option<usize>> {
        let graphs_count_opt = self.config.get("number-of-graphs");
        let graphs_count;
        if graphs_count_opt.is_none() {
            graphs_count = None;
        } else {
            graphs_count = Option::Some(
                u64::from_str(graphs_count_opt.unwrap().clone().as_str()).unwrap() as usize,
            );
        }
        Ok(graphs_count)
    }

    pub fn get_colouriser_type(&self) -> Result<Option<&String>> {
        let colouriser_type_opt = self.config.get("colouriser-type");
        let colouriser_type;
        if colouriser_type_opt.is_none() {
            colouriser_type = None;
        } else {
            colouriser_type = Option::Some(colouriser_type_opt.unwrap());
        }
        Ok(colouriser_type)
    }
}
