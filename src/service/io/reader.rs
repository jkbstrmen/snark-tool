use crate::service::io::error::ReadError;
use std::fs;

pub trait Reader<'a, G> {
    fn new(file: &'a fs::File) -> Self;

    // TODO
    // fn from_file(file: &'a fs::File) -> Self;
    // fn from_path(path: impl AsRef<str>) -> Self;

    fn next(&mut self) -> Option<Result<G, ReadError>>;
}
