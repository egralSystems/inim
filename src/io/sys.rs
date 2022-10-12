use std::prelude::v1::*;
use rhai::Dynamic;

pub trait Sys: Clone {
    fn ls(path: &str) -> Vec<Dynamic>;
    fn mkdir(path: &str) -> bool;
    fn rm(path: &str) -> bool;
    fn rmdir(path: &str) -> bool;
    fn time() -> f64;
    fn path() -> String;
}
