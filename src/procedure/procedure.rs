use crate::error::Error;
use crate::graph::traits::graph::Graph;
use std::collections::HashMap;
use std::{fmt, result};

type Result<T> = result::Result<T, Error>;
pub type Config = HashMap<String, String>;

pub trait Procedure {
    fn new_with_config(proc_type: impl AsRef<str>, config: Config) -> Self;

    fn run<G>(&self, graphs: &mut Vec<G>) -> Result<()>
    where
        G: fmt::Debug + Graph;
}
