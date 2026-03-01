//! RISC-V register file management.
//!
//! This module provides the [`Register`] struct which manages the 32 general-purpose
//! registers and program counter for the RISC-V processor.

/// The register file for a RISC-V processor.
///
/// Maintains 32 general-purpose 32-bit registers and a program counter.
/// Register x0 is special and always returns 0 (attempts to write to it are ignored).
///
/// # Register Layout
///
/// - **x0-x31**: 32 general-purpose 32-bit registers
/// - **PC**: Program counter tracking instruction execution
///
/// # Register x0 (Zero Register)
///
/// The x0 register is hard-wired to zero. Reads from x0 always return 0,
/// and writes to x0 are silently ignored (actually panic). This is a
/// RISC-V specification requirement.
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
    /// # Returns
    ///
    /// A register file with:
    /// - All 32 registers (x0-x31) set to 0
    /// - Program counter set to 0
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
    /// It typically increments by 4 after each instruction (since instructions are 32-bit).
    ///
    /// # Returns
    ///
    /// The current program counter value (byte address in memory).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use risc_v_emulator::processor::Processor;
    ///
    /// let proc = Processor::new();
    /// assert_eq!(proc.get_registry_value(0), 0);  // PC starts at 0
    /// ```
    pub fn pc(&self) -> usize {
        self._pc
    }

    /// Updates the program counter to a new value.
    ///
    /// # Arguments
    ///
    /// * `pc` - The new program counter value
    ///
    /// # Example
    ///
    /// ```no_run
    /// use risc_v_emulator::processor::Processor;
    ///
    /// let mut proc = Processor::new();
    /// // After executing an instruction, PC advances
    /// // let pc = proc.pc() + 4;
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
    /// - Panics if `index` is 0 (x0 is read-only)
    /// - Panics if `index` > 31 (only 32 registers exist)
    ///
    /// # Example
    ///
    /// ```
    /// use risc_v_emulator::processor::Processor;
    ///
    /// let mut proc = Processor::new();
    /// proc.set_register_value(5, 100);  // Uses this internally
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
    /// The 32-bit value in the register. For x0, always returns 0.
    ///
    /// # Panics
    ///
    /// Panics if `index` > 31 (only 32 registers exist).
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
