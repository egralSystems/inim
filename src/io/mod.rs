// Standardized I/O interfaces
pub mod console;
pub mod fs;
pub mod net;
pub mod sys;

pub mod default_io;
// Default I/O implementations
pub mod dummy;

#[cfg(feature = "std")]
pub mod standard;
