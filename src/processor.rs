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
/// # Debug Output
///
/// The `execute_instructions()` method prints detailed debug information showing:
/// - Each instruction being executed
/// - The decoded instruction parameters
/// - The state of all registers after each instruction
///
/// This is useful for understanding program flow and debugging assembly code.
///
/// # Panics and Error Handling
///
/// The processor will panic in these situations:
/// - Attempting to write to the x0 register (zero register)
/// - Invalid instruction encoding
/// - Out-of-bounds memory access
/// - Invalid register indices (> 31)
///
/// # Example: Simple Arithmetic
///
/// ```no_run
/// use risc_v_emulator::processor::Processor;
/// use risc_v_emulator::assembler::assemble;
///
/// // Create a new processor
/// let mut processor = Processor::new();
///
/// // Assemble and load instructions
/// let assembly = vec![
///     "addi x5, x0, 42".to_string(),
///     "addi x6, x0, 8".to_string(),
///     "add x7, x5, x6".to_string(),
/// ];
/// processor.load_instructions(assemble(assembly));
///
/// // Execute all instructions
/// processor.execute_instructions();
///
/// // Check results
/// assert_eq!(processor.get_registry_value(5), 42);
/// assert_eq!(processor.get_registry_value(6), 8);
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
///
/// // Pre-load some data into memory
/// let data = vec![0xDEu8, 0xADu8, 0xBEu8, 0xEFu8];
/// processor.load_into_memory(&data);
///
/// // Assemble and load instructions to read that data
/// let assembly = vec![
///     "lw x8, 512".to_string(),  // Load word from address 512
/// ];
/// processor.load_instructions(assemble(assembly));
/// processor.execute_instructions();
///
/// // Read the loaded value
/// let value = processor.get_registry_value(8);
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
    /// The stack pointer is placed at address 256, which is in the middle of the available
    /// memory. This allows for both growing stacks and data storage in different regions.
    ///
    /// # Complexity
    ///
    /// - Time: O(1) - initializes fixed-size register and memory arrays
    /// - Space: O(1) - constant memory for a processor instance
    ///
    /// # Example
    ///
    /// ```
    /// use risc_v_emulator::processor::Processor;
    ///
    /// let processor = Processor::new();
    ///
    /// // Verify initialization
    /// assert_eq!(processor.get_registry_value(0), 0);   // x0 (zero register)
    /// assert_eq!(processor.get_registry_value(1), 0);   // x1 (return address)
    /// assert_eq!(processor.get_registry_value(2), 256); // x2 (stack pointer)
    /// assert_eq!(processor.get_registry_value(5), 0);   // General register
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

    /// Loads assembled instructions into processor memory.
    ///
    /// Takes a vector of 32-bit instruction words and stores them in memory
    /// starting at address 0. Each instruction occupies 4 bytes in big-endian format.
    ///
    /// # Arguments
    ///
    /// * `executable_code` - A vector of 32-bit instruction words to load
    ///
    /// # Panics
    ///
    /// Panics if the executable code would overflow the 1024-byte memory space.
    /// Maximum of 256 instructions can be loaded.
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
    pub fn load_instructions(&mut self, executable_code: Vec<u32>) -> () {
        for (idx, instruction) in executable_code.iter().enumerate() {
            self.memory[idx * 4..idx * 4 + 4].copy_from_slice(&instruction.to_be_bytes())
        }
    }

    /// Loads raw bytes into the data memory region.
    ///
    /// Copies bytes from the provided slice into processor memory starting at
    /// address 512 (data memory region). This is useful for pre-loading data
    /// or memory-mapped state that programs will access.
    ///
    /// # Arguments
    ///
    /// * `src` - Slice of bytes to copy into memory
    ///
    /// # Returns
    ///
    /// Returns the memory address (512) where the data was loaded.
    ///
    /// # Panics
    ///
    /// Panics if the source data would overflow the available data memory
    /// (512 bytes from address 512-1023).
    ///
    /// # Example
    ///
    /// ```
    /// use risc_v_emulator::processor::Processor;
    ///
    /// let mut processor = Processor::new();
    /// let data = vec![42u8, 100u8, 255u8];
    /// let addr = processor.load_into_memory(&data);
    /// assert_eq!(addr, 512);  // Data loaded at address 512
    /// ```
    pub fn load_into_memory(&mut self, src: &[u8]) -> usize {
        let len = src.len();
        self.memory[512..512 + len].copy_from_slice(src);
        512
    }

    /// Sets the value of a register.
    ///
    /// Updates the value of the register at the specified index.
    /// Register x0 (index 0) is special and cannot be modified - attempting to
    /// write to it will cause a panic.
    ///
    /// # Arguments
    ///
    /// * `index` - The register index (0-31)
    /// * `value` - The 32-bit value to set
    ///
    /// # Panics
    ///
    /// - Panics if `index` is 0 (x0 is read-only and hard-wired to zero)
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
    pub fn set_register_value(&mut self, index: usize, value: u32) {
        self.register.put(index, value);
    }

    /// Executes loaded instructions sequentially.
    ///
    /// Starts execution from the current program counter (usually 0 after loading instructions)
    /// and executes instructions one at a time in a fetch-decode-execute loop.
    /// Each instruction is 4 bytes, so the PC increments by 4 after each instruction.
    ///
    /// Execution continues until an instruction signals termination. The specific
    /// termination condition is determined by the instruction's `should_end()` method.
    ///
    /// This method prints debug information during execution showing each instruction
    /// fetched, decoded, and the resulting register state.
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
    /// let code = assemble(asm);
    /// processor.load_instructions(code);
    /// processor.execute_instructions();  // Runs until completion
    /// ```
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

    /// Retrieves a copy of memory contents from a specified range.
    ///
    /// Returns a vector containing a copy of the bytes in the specified memory range.
    /// This allows inspection of memory without modifying the processor state.
    ///
    /// # Arguments
    ///
    /// * `range` - A range of memory addresses to copy (e.g., `512..520`)
    ///
    /// # Returns
    ///
    /// A vector containing a copy of the bytes in the specified range.
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
    pub fn get_copy_of_memory(&mut self, range: Range<usize>) -> Vec<u8> {
        self.memory[range].to_owned()
    }

    /// Gets the current value of a register.
    ///
    /// Returns the 32-bit value stored in the register at the specified index.
    /// Reading from x0 always returns 0, regardless of any attempted writes.
    ///
    /// # Arguments
    ///
    /// * `index` - The register index (0-31)
    ///
    /// # Returns
    ///
    /// The 32-bit value in the register.
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
    pub fn get_registry_value(&self, index: usize) -> u32 {
        self.register.get(index)
    }
}
