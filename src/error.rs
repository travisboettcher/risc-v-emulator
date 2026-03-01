use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmulatorError {
    #[error("Memory access out of bounds: address {address:#x}, memory size {size}")]
    MemoryOutOfBounds { address: usize, size: usize },

    #[error("Invalid instruction: {0:#x}")]
    InvalidInstruction(u32),

    #[error("Unaligned memory access: address {address:#x}, required alignment {alignment}")]
    UnalignedAccess { address: usize, alignment: usize },

    #[error("Invalid register index: {0} (must be 0-31)")]
    InvalidRegister(usize),

    #[error("Attempt to modify read-only register x0 (zero register)")]
    ModifyZeroRegister,

    #[error("Division by zero at PC {0:#x}")]
    DivisionByZero(usize),

    #[error("Program counter out of bounds: {pc:#x}, memory size {size}")]
    ProgramCounterOutOfBounds { pc: usize, size: usize },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, EmulatorError>;
