//! RISC-V CPU processor simulator.
//!
//! This module provides the [`Processor`] struct which implements a complete RISC-V
//! CPU simulator. The processor manages 32 general-purpose registers, a 1024-byte memory space,
//! and executes 32-bit RISC-V instructions in a fetch-decode-execute cycle.
//!
//! # Quick Start
//!
//! ```no_run
//! use risc_v_emulator::processor::Processor;
//! use risc_v_emulator::assembler::assemble;
//!
//! let mut processor = Processor::new();
//! let code = assemble(vec!["addi x5, x0, 42".to_string()]);
//! processor.load_instructions(code);
//! processor.execute_instructions();
//! assert_eq!(processor.get_registry_value(5), 42);
//! ```

use crate::register::Register;
use crate::instruction;
use std::ops::Range;

const SP: usize = 2;

/// A RISC-V processor simulator.
///
/// The `Processor` struct represents a complete RISC-V CPU that can execute 32-bit instructions.
/// It manages 32 general-purpose registers and a 1024-byte memory space, along with
/// a program counter (PC) for tracking instruction execution.
///
/// This is the main interface for executing RISC-V assembly programs. To use the processor:
/// 1. Create a new instance with `Processor::new()`
/// 2. Load assembled instructions with `load_instructions()`
/// 3. Optionally pre-load data with `load_into_memory()`
/// 4. Execute with `execute_instructions()`
/// 5. Read results with `get_registry_value()` and `get_copy_of_memory()`
///
/// # Memory Layout
///
/// The processor has 1024 bytes of total addressable memory, split into regions:
///
/// | Address Range | Size | Purpose |
/// |---------------|------|---------|
/// | 0-511 | 512 bytes | Instruction memory (loaded via `load_instructions`) |
/// | 512-1023 | 512 bytes | Data memory (accessed via load/store instructions) |
/// | Stack (x2) | Variable | Stack grows downward from initialization point |
///
/// The instruction memory can hold up to 256 instructions (256 × 4 bytes).
/// The data memory is available for general-purpose data storage and stack operations.
///
/// # Register Management
///
/// The processor maintains 32 general-purpose 32-bit registers (x0-x31):
///
/// | Register | ABI Name | Purpose | Special |
/// |----------|----------|---------|---------|
/// | x0 | zero | Always returns 0 | Hard-wired, writes cause panic |
/// | x1 | ra | Return address | Used by `jal` and `jalr` instructions |
/// | x2 | sp | Stack pointer | Initialized to address 256 |
/// | x3-x31 | | General-purpose | Free to use for any purpose |
///
/// # Execution Model
///
/// The processor implements a classic fetch-decode-execute cycle:
///
/// 1. **Fetch**: Retrieve instruction from memory at program counter (PC)
/// 2. **Decode**: Parse the 32-bit instruction into operation and operands
/// 3. **Execute**: Perform the operation (modify registers/memory)
/// 4. **Update PC**: Increment PC by 4 (or branch to a different address)
///
/// Execution starts at memory address 0 and continues until an instruction
/// signals termination via `should_end()`. The program counter increments by 4
/// after each instruction (since all RISC-V instructions are 32 bits / 4 bytes).
///
/// # Example: Simple Arithmetic
///
/// ```no_run
/// use risc_v_emulator::processor::Processor;
/// use risc_v_emulator::assembler::assemble;
///
/// let mut processor = Processor::new();
/// let assembly = vec![
///     "addi x5, x0, 42".to_string(),
///     "addi x6, x0, 8".to_string(),
///     "add x7, x5, x6".to_string(),
/// ];
/// processor.load_instructions(assemble(assembly));
/// processor.execute_instructions();
/// assert_eq!(processor.get_registry_value(7), 50);
/// ```
///
/// # Example: Memory Operations
///
/// ```no_run
/// use risc_v_emulator::processor::Processor;
/// use risc_v_emulator::assembler::assemble;
///
/// let mut processor = Processor::new();
/// let data = vec![0xDEu8, 0xADu8, 0xBEu8, 0xEFu8];
/// processor.load_into_memory(&data);
///
/// let assembly = vec!["lw x8, 512".to_string()];
/// processor.load_instructions(assemble(assembly));
/// processor.execute_instructions();
/// ```
pub struct Processor {
    register: Register,
    memory: [u8; 1024]
}

impl Processor {
    /// Creates a new processor instance with all resources initialized.
    ///
    /// Constructs a new `Processor` with a clean state ready for loading and executing code.
    /// All registers are initialized to zero, except the stack pointer (x2) which is set to
    /// address 256. All memory (both instruction and data regions) is zeroed.
    ///
    /// # State After Creation
    ///
    /// - **All registers x0-x31**: 0 (x0 is special and always returns 0)
    /// - **Program Counter (PC)**: 0 (starts at first instruction)
    /// - **Stack Pointer (x2)**: 256 (in the middle of memory space)
    /// - **Memory**: All 1024 bytes set to 0
    ///
    /// # Example
    ///
    /// ```
    /// use risc_v_emulator::processor::Processor;
    ///
    /// let processor = Processor::new();
    /// assert_eq!(processor.get_registry_value(0), 0);   // x0
    /// assert_eq!(processor.get_registry_value(2), 256); // sp
    /// ```
    pub fn new() -> Processor {
        let mut proc = Processor {
            register: Register::new(),
            memory: [0u8; 1024]
        };

        // Initialize stack pointer to memory address 256
        proc.set_register_value(SP, 256);
        proc
    }

    /// Loads assembled instructions into processor instruction memory.
    ///
    /// Takes a vector of 32-bit instruction words and stores them in memory
    /// starting at address 0 (the instruction memory region). Each instruction occupies
    /// exactly 4 bytes in big-endian (network byte order) format.
    ///
    /// # Arguments
    ///
    /// * `executable_code` - A vector of 32-bit instruction words to load
    ///
    /// # Panics
    ///
    /// Panics if the executable code would overflow the instruction memory region.
    /// Maximum of 128 instructions can be loaded (512 bytes / 4 bytes per instruction).
    ///
    /// # Example
    ///
    /// ```
    /// use risc_v_emulator::processor::Processor;
    ///
    /// let mut processor = Processor::new();
    /// let instructions = vec![
    ///     0b00000000000100000000001010010011,  // addi x5, x0, 1
    ///     0b00000000001000000000001100010011,  // addi x6, x0, 2
    /// ];
    /// processor.load_instructions(instructions);
    /// ```
    ///
    /// # See Also
    ///
    /// - [`load_into_memory`](Self::load_into_memory) - Load data into data memory region
    /// - [`execute_instructions`](Self::execute_instructions) - Execute loaded instructions
    pub fn load_instructions(&mut self, executable_code: Vec<u32>) -> () {
        for (idx, instruction) in executable_code.iter().enumerate() {
            self.memory[idx * 4..idx * 4 + 4].copy_from_slice(&instruction.to_be_bytes())
        }
    }

    /// Loads raw bytes into the data memory region.
    ///
    /// Copies bytes from the provided slice into processor memory starting at
    /// address 512 (the data memory region). This is the primary method for pre-loading
    /// data that programs will access via load/store instructions.
    ///
    /// # Arguments
    ///
    /// * `src` - A slice of bytes to copy into memory. Can be any length up to 512 bytes.
    ///
    /// # Returns
    ///
    /// Always returns the address `512`, which is where the data was loaded.
    ///
    /// # Panics
    ///
    /// Panics if the source data size exceeds 512 bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use risc_v_emulator::processor::Processor;
    ///
    /// let mut processor = Processor::new();
    /// let data = vec![42u8, 100u8, 255u8];
    /// let addr = processor.load_into_memory(&data);
    /// assert_eq!(addr, 512);
    ///
    /// let retrieved = processor.get_copy_of_memory(512..515);
    /// assert_eq!(retrieved, data);
    /// ```
    ///
    /// # See Also
    ///
    /// - [`load_instructions`](Self::load_instructions) - Load code into instruction memory
    /// - [`get_copy_of_memory`](Self::get_copy_of_memory) - Read memory contents
    pub fn load_into_memory(&mut self, src: &[u8]) -> usize {
        let len = src.len();
        self.memory[512..512 + len].copy_from_slice(src);
        512
    }

    /// Sets the value of a general-purpose register.
    ///
    /// Updates the 32-bit value stored in the register at the specified index.
    /// Register x0 (index 0) cannot be modified - writes to x0 will cause a panic.
    ///
    /// # Arguments
    ///
    /// * `index` - The register index (0-31, where 0 is the zero register)
    /// * `value` - The 32-bit unsigned integer value to store in the register
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
    /// let mut processor = Processor::new();
    /// processor.set_register_value(5, 42);
    /// assert_eq!(processor.get_registry_value(5), 42);
    /// ```
    ///
    /// # See Also
    ///
    /// - [`get_registry_value`](Self::get_registry_value) - Read a register's value
    pub fn set_register_value(&mut self, index: usize, value: u32) {
        self.register.put(index, value);
    }

    /// Executes all loaded instructions in a fetch-decode-execute loop.
    ///
    /// Starts execution from the current program counter (typically 0 after loading instructions)
    /// and executes instructions sequentially until an instruction signals termination.
    ///
    /// This method prints debug information during execution showing each instruction
    /// fetched, decoded, and the resulting register state. This is useful for:
    /// - Tracing program execution
    /// - Debugging assembly code logic
    /// - Understanding instruction effects
    /// - Verifying register state changes
    ///
    /// # Panics
    ///
    /// - Panics if instruction decoding fails
    /// - Panics if the program counter goes out of bounds
    /// - Panics if an instruction attempts invalid register operations
    ///
    /// # Example
    ///
    /// ```no_run
    /// use risc_v_emulator::processor::Processor;
    /// use risc_v_emulator::assembler::assemble;
    ///
    /// let mut processor = Processor::new();
    /// let asm = vec!["addi x5, x0, 10".to_string()];
    /// processor.load_instructions(assemble(asm));
    /// processor.execute_instructions();  // Runs until completion
    /// ```
    ///
    /// # See Also
    ///
    /// - [`load_instructions`](Self::load_instructions) - Load code before execution
    /// - [`get_registry_value`](Self::get_registry_value) - Read results after execution
    /// - [`get_copy_of_memory`](Self::get_copy_of_memory) - Inspect memory after execution
    pub fn execute_instructions(&mut self) {
        println!("--------------------------");
        loop {
            let mut binary= [0u8; 4];
            binary.copy_from_slice(&self.memory[self.register.pc()..self.register.pc() + 4]);
            // println!("[executing] Input: {:0>32b}", binary);
            let instruction = instruction::from(binary).unwrap();
            println!("[executing] Instruction: {:?}", instruction);

            if instruction.should_end(&self.register) {
                break;
            }

            self.register.update_pc(self.register.pc() + 4);

            instruction.execute(&mut self.register, &mut self.memory);
            println!("[executing] Register: {:?}", self.register);
            println!("--------------------------");
        }
    }

    /// Retrieves a copy of memory contents from a specified address range.
    ///
    /// Returns a vector containing a copy of the bytes in the specified memory range.
    /// This is the primary method for inspecting memory after execution without modifying
    /// the processor state.
    ///
    /// # Arguments
    ///
    /// * `range` - A range of byte addresses (e.g., `512..520`)
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing a copy of all bytes in the specified range.
    ///
    /// # Panics
    ///
    /// Panics if the range exceeds the 1024-byte memory space.
    ///
    /// # Example
    ///
    /// ```
    /// use risc_v_emulator::processor::Processor;
    ///
    /// let mut processor = Processor::new();
    /// let data = vec![10u8, 20u8, 30u8];
    /// processor.load_into_memory(&data);
    ///
    /// let retrieved = processor.get_copy_of_memory(512..515);
    /// assert_eq!(retrieved, vec![10, 20, 30]);
    /// ```
    ///
    /// # See Also
    ///
    /// - [`load_into_memory`](Self::load_into_memory) - Write data to memory
    /// - [`get_registry_value`](Self::get_registry_value) - Read register values
    pub fn get_copy_of_memory(&mut self, range: Range<usize>) -> Vec<u8> {
        self.memory[range].to_owned()
    }

    /// Gets the current 32-bit value of a register.
    ///
    /// Returns the 32-bit unsigned integer value stored in the register at the specified index.
    /// Reading from x0 always returns 0, regardless of any attempted writes.
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
    /// let mut processor = Processor::new();
    /// processor.set_register_value(5, 42);
    /// assert_eq!(processor.get_registry_value(5), 42);
    ///
    /// // x0 always returns 0
    /// assert_eq!(processor.get_registry_value(0), 0);
    /// ```
    ///
    /// # See Also
    ///
    /// - [`set_register_value`](Self::set_register_value) - Set a register's value
    /// - [`get_copy_of_memory`](Self::get_copy_of_memory) - Read memory contents
    pub fn get_registry_value(&self, index: usize) -> u32 {
        self.register.get(index)
    }
}
