use crate::service::io::error::ReadError;
use std::fs;

pub trait Reader<'a, G> {
    fn new(file: &'a fs::File) -> Self;
    fn next(&mut self) -> Option<Result<G, ReadError>>;
}
