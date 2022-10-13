// Standardized I/O interfaces
pub mod console;
pub mod fs;
pub mod net;
pub mod sys;

// Default I/O implementations
pub mod dummy;

#[cfg(feature = "std")]
pub mod standard;
