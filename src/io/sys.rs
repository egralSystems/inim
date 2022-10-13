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
