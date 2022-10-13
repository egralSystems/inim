#![no_std]

extern crate no_std_compat as std;

pub mod inim;
pub mod io;
mod module_resolver;

pub mod prelude {
    pub use crate::inim::*;
    pub use crate::io::*;
}
