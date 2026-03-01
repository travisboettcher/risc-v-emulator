//! RISC-V register file management.
//!
//! This module provides the [`Register`] struct which manages the 32 general-purpose
//! registers and program counter for the RISC-V processor.
//!
//! # Register File Architecture
//!
//! The register file implements the RISC-V integer register specification with:
//! - **32 General-Purpose Registers**: x0-x31, each 32 bits
//! - **Program Counter**: Tracks the current instruction address
//! - **Zero Register (x0)**: Hard-wired to always return 0, writes are rejected
//!
//! # Register Mapping
//!
//! | Index | ABI Name | Purpose | Special |
//! |-------|----------|---------|---------|
//! | 0 | zero | Hard-wired zero | Read-only |
//! | 1 | ra | Return address | Used by JAL/JALR |
//! | 2 | sp | Stack pointer | Initialized to 256 |
//! | 3 | gp | Global pointer | |
//! | 4 | tp | Thread pointer | |
//! | 5-7 | t0-t2 | Temporary | Caller-saved |
//! | 8 | s0/fp | Saved/Frame pointer | Callee-saved |
//! | 9 | s1 | Saved | Callee-saved |
//! | 10-11 | a0-a1 | Arguments/Return | Caller-saved |
//! | 12-17 | a2-a7 | Arguments | Caller-saved |
//! | 18-27 | s2-s11 | Saved | Callee-saved |
//! | 28-31 | t3-t6 | Temporary | Caller-saved |
//!
//! # Usage Notes
//!
//! The register file is an internal component typically used by the [`crate::processor::Processor`].
//! Direct interaction with the register file is usually done through processor methods
//! like [`crate::processor::Processor::set_register_value`] and
//! [`crate::processor::Processor::get_registry_value`].
//!
//! # Example
//!
//! ```
//! use risc_v_emulator::processor::Processor;
//!
//! let mut proc = Processor::new();
//! proc.set_register_value(5, 42);
//! assert_eq!(proc.get_registry_value(5), 42);
//! assert_eq!(proc.get_registry_value(0), 0);  // x0 always zero
//! ```

/// The register file for a RISC-V processor.
///
/// Maintains 32 general-purpose 32-bit registers and a program counter.
/// Register x0 is special and always returns 0 (attempts to write to it cause a panic).
///
/// This struct is typically used internally by the [`crate::processor::Processor`] and
/// is not meant for direct public use. Access to registers should go through the processor's
/// public interface.
///
/// # Register Layout
///
/// - **x0-x31**: 32 general-purpose 32-bit registers
/// - **PC**: Program counter tracking instruction execution
///
/// # Register x0 (Zero Register)
///
/// The x0 register is hard-wired to zero. Reads from x0 always return 0,
/// and writes to x0 will cause a panic. This is a fundamental RISC-V
/// specification requirement.
///
/// # Example
///
/// ```
/// use risc_v_emulator::processor::Processor;
///
/// let mut proc = Processor::new();
/// proc.set_register_value(5, 42);
/// assert_eq!(proc.get_registry_value(5), 42);
/// assert_eq!(proc.get_registry_value(0), 0);  // x0 always zero
/// ```
#[derive(Debug)]
pub struct Register {
    _x: [u32; 32],
    _pc: usize
}

impl Register {
    /// Creates a new register file with all registers initialized to zero.
    ///
    /// Initializes the register file for a RISC-V processor with:
    /// - All 32 registers (x0-x31) set to 0
    /// - Program counter set to 0
    ///
    /// This is typically called by [`crate::processor::Processor::new`] to initialize
    /// the processor state.
    ///
    /// # Returns
    ///
    /// A new [`Register`] struct with:
    /// - All 32 registers (x0-x31) set to 0
    /// - Program counter (PC) set to 0
    ///
    /// # Complexity
    ///
    /// - Time: O(1) - Fixed-size initialization
    /// - Space: O(1) - Fixed-size register array
    ///
    /// # Example
    ///
    /// ```
    /// use risc_v_emulator::processor::Processor;
    ///
    /// let proc = Processor::new();
    /// assert_eq!(proc.get_registry_value(0), 0);  // All registers initialized
    /// ```
    pub fn new() -> Register {
        Register {
            _x: [0; 32],
            _pc: 0
        }
    }

    /// Retrieves the current program counter value.
    ///
    /// The program counter tracks which instruction is being executed.
    /// It points to a byte address in instruction memory (typically increments by 4
    /// after each instruction since all RISC-V instructions are 32-bit).
    ///
    /// # Returns
    ///
    /// The current program counter value (byte address in memory).
    /// Initially 0, can range from 0 to 1023 (the 1024-byte memory space).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use risc_v_emulator::processor::Processor;
    ///
    /// let proc = Processor::new();
    /// // PC starts at 0 before any instructions execute
    /// ```
    pub fn pc(&self) -> usize {
        self._pc
    }

    /// Updates the program counter to a new value.
    ///
    /// Sets the program counter to a new address. This is typically called to:
    /// - Advance to the next instruction (increment by 4)
    /// - Branch to a different instruction address
    /// - Jump to a subroutine
    ///
    /// # Arguments
    ///
    /// * `pc` - The new program counter value (byte address)
    ///
    /// # Side Effects
    ///
    /// - Updates the program counter to the new value
    /// - The next instruction fetched will be from this address
    /// - No bounds checking is performed (out-of-bounds will panic during execution)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use risc_v_emulator::processor::Processor;
    ///
    /// let mut proc = Processor::new();
    /// // Processor internally updates PC during instruction execution
    /// // Normal incrementing: PC += 4 after each instruction
    /// // Branches/jumps modify PC to jump to different locations
    /// ```
    pub fn update_pc(&mut self, pc: usize) {
        self._pc = pc;
    }

    /// Writes a value to the specified register.
    ///
    /// Sets the 32-bit value for the register at the given index.
    /// This is the internal implementation used by the processor.
    ///
    /// # Arguments
    ///
    /// * `index` - The register index (0-31)
    /// * `value` - The 32-bit value to write
    ///
    /// # Panics
    ///
    /// - **Panics if `index` is 0**: x0 is read-only per RISC-V specification
    ///   - Error message: "Cannot modify the register at index 0"
    /// - **Panics if `index` > 31**: Only 32 registers exist
    ///   - Error message: "The register only has a length of 32, tried to modify index {index}"
    ///
    /// # Register Behavior
    ///
    /// - x0 (zero register): Always reads as 0, any write attempt panics
    /// - x1-x31: Standard 32-bit integer registers
    ///
    /// # Example
    ///
    /// ```
    /// use risc_v_emulator::processor::Processor;
    ///
    /// let mut proc = Processor::new();
    /// proc.set_register_value(5, 100);  // Uses this internally
    /// assert_eq!(proc.get_registry_value(5), 100);
    /// ```
    pub fn put(&mut self, index: usize, value: u32) {
        if index == 0 {
            panic!("Cannot modify the register at index 0");
        }

        if index > 31 {
            panic!("The register only has a length of 32, tried to modify index {}", index)
        }

        self._x[index] = value;
    }

    /// Reads the value from the specified register.
    ///
    /// Retrieves the current 32-bit value stored in the register at the given index.
    /// This is the internal implementation used by the processor.
    ///
    /// # Arguments
    ///
    /// * `index` - The register index (0-31)
    ///
    /// # Returns
    ///
    /// The 32-bit unsigned integer value in the register.
    /// For x0, always returns 0 regardless of attempted writes.
    ///
    /// # Panics
    ///
    /// Panics if `index` > 31:
    /// - Only 32 registers exist (x0 through x31)
    /// - Error message: "The register only has a length of 32, tried to access index {index}"
    ///
    /// # Register Behavior
    ///
    /// - x0 (zero register): Always returns 0
    /// - x1-x31: Returns the stored 32-bit value
    ///
    /// # Complexity
    ///
    /// - Time: O(1) - Direct array access
    /// - Space: O(1) - No allocation
    ///
    /// # Example
    ///
    /// ```
    /// use risc_v_emulator::processor::Processor;
    ///
    /// let proc = Processor::new();
    /// let val = proc.get_registry_value(5);  // Uses this internally
    /// ```
    pub fn get(&self, index: usize) -> u32 {
        if index > 31 {
            panic!("The register only has a length of 32, tried to access index {}", index)
        }

        self._x[index]
    }
}
