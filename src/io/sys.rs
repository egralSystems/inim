use rhai::Dynamic;
use std::prelude::v1::*;

pub trait Sys: Clone + 'static {
    fn ls(path: &str) -> Vec<Dynamic>;
    fn mkdir(path: &str) -> bool;
    fn rm(path: &str) -> bool;
    fn rmdir(path: &str) -> bool;
    fn time() -> f64;
    fn path() -> String;
}

#[derive(Clone)]
pub struct DummySys;

impl Sys for DummySys {
    fn ls(_path: &str) -> Vec<Dynamic> {
        vec![]
    }

    fn mkdir(_path: &str) -> bool {
        false
    }

    fn rm(_path: &str) -> bool {
        false
    }

    fn rmdir(_path: &str) -> bool {
        false
    }

    fn time() -> f64 {
        0.0
    }

    fn path() -> String {
        "".to_string()
    }
}
