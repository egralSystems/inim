use rhai::{Dynamic, Engine};
use std::prelude::v1::*;

pub fn register_sys<S: Sys>(engine: &mut Engine) {
    engine
        .register_fn("ls", S::ls)
        .register_fn("rm", S::rm)
        .register_fn("mkdir", S::mkdir)
        .register_fn("rmdir", S::rmdir)
        .register_fn("time", S::time)
        .register_fn("path", S::path);
}

pub trait Sys: Clone + 'static {
    fn ls(path: &str) -> Vec<Dynamic>;
    fn mkdir(path: &str) -> bool;
    fn rm(path: &str) -> bool;
    fn rmdir(path: &str) -> bool;
    fn time() -> f64;
    fn path() -> String;
}
