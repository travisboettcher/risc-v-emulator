mod register;
mod instruction;
mod math_utils;
pub mod assembler;
mod immediates;
pub mod processor;

// Re-export ProcessorBuilder for convenient access
pub use processor::ProcessorBuilder;
