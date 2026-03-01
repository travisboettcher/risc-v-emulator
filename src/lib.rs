//! # RISC-V Emulator
//!
//! A comprehensive RISC-V instruction set emulator written in Rust.
//! This library provides a fully functional RISC-V processor simulator capable of
//! executing assembly code, managing 32 general-purpose registers, and handling
//! a 1024-byte memory space.
//!
//! ## Features
//!
//! - **Full RISC-V Instruction Support**: I-type, R-type, S-type, B-type, U-type, and J-type instruction formats
//! - **32 General-Purpose Registers**: x0-x31 with special x0 (zero) register behavior
//! - **1024-Byte Memory**: Combined instruction and data memory with proper segmentation
//! - **Pseudo-Instruction Expansion**: Automatic expansion of pseudo-instructions like `nop`, `li`, `mv`, `j`
//! - **Symbol Table Management**: Full label and forward/backward reference support for assembly code
//! - **Comment Handling**: Inline and line-based comment support in assembly
//! - **Instruction Execution**: Fetch-decode-execute cycle with proper program counter management
//!
//! ## Quick Start
//!
//! The basic workflow involves three steps: write assembly, assemble to machine code, and execute.
//!
//! ### Simple Arithmetic Example
//!
//! ```no_run
//! use risc_v_emulator::assembler::assemble;
//! use risc_v_emulator::processor::Processor;
//!
//! // Write assembly code
//! let assembly = vec![
//!     "addi x5, x0, 10".to_string(),   // Load 10 into x5
//!     "addi x6, x0, 20".to_string(),   // Load 20 into x6
//!     "add x7, x5, x6".to_string(),    // Add x5 and x6, store in x7
//! ];
//!
//! // Assemble to machine code
//! let executable = assemble(assembly);
//!
//! // Create processor and execute
//! let mut processor = Processor::new();
//! processor.load_instructions(executable);
//! processor.execute_instructions();
//!
//! // Read results
//! assert_eq!(processor.get_registry_value(7), 30);
//! ```
//!
//! ### With Memory Operations
//!
//! ```no_run
//! use risc_v_emulator::assembler::assemble;
//! use risc_v_emulator::processor::Processor;
//!
//! let assembly = vec![
//!     "addi x5, x0, 100".to_string(),   // Load 100 into x5
//!     "sw x5, 512".to_string(),         // Store x5 to memory address 512
//!     "lw x6, 512".to_string(),         // Load from memory address 512 into x6
//! ];
//!
//! let mut processor = Processor::new();
//! processor.load_instructions(assemble(assembly));
//! processor.execute_instructions();
//! ```
//!
//! ## Architecture
//!
//! The emulator is structured into several core modules:
//!
//! - [`processor`] - Main CPU simulator handling instruction execution and memory management
//! - [`assembler`] - Assembly language compiler and pseudo-instruction expansion
//!
//! ## Supported Instructions
//!
//! ### Arithmetic & Logic (R-type)
//!
//! Perform operations on two source registers and write result to destination:
//! `add`, `sub`, `and`, `or`, `xor`, `sll`, `srl`, `sra`, `slt`, `sltu`
//!
//! ### Arithmetic & Logic Immediate (I-type)
//!
//! Perform operations on a register and an immediate value:
//! `addi`, `andi`, `ori`, `xori`, `slli`, `srli`, `srai`, `slti`, `sltiu`
//!
//! ### Load Instructions (I-type)
//!
//! Load data from memory into registers:
//! - `lw` - Load word (32-bit)
//! - `lh` - Load half-word (16-bit, sign-extended)
//! - `lhu` - Load half-word unsigned (16-bit, zero-extended)
//! - `lb` - Load byte (8-bit, sign-extended)
//! - `lbu` - Load byte unsigned (8-bit, zero-extended)
//!
//! ### Store Instructions (S-type)
//!
//! Store register data to memory:
//! - `sw` - Store word (32-bit)
//! - `sh` - Store half-word (16-bit)
//! - `sb` - Store byte (8-bit)
//!
//! ### Branch Instructions (B-type)
//!
//! Conditional branches based on register comparison:
//! `beq`, `bne`, `blt`, `bge`, `bltu`, `bgeu`
//!
//! ### Upper Immediate (U-type)
//!
//! Load 20-bit immediate into upper bits:
//! `lui` - Load upper immediate
//! `auipc` - Add upper immediate to program counter
//!
//! ### Jump Instructions (J-type)
//!
//! Unconditional jumps with return address handling:
//! - `jal` - Jump and link (saves return address)
//! - `jalr` - Jump and link register
//!
//! ### Pseudo-Instructions
//!
//! Higher-level instructions automatically expanded to base instructions:
//! `nop`, `li`, `mv`, `not`, `neg`, `seqz`, `snez`, `sltz`, `sgtz`,
//! `beqz`, `bnez`, `bgt`, `ble`, `j`, `ret`, `call`
//!
//! ## Instruction Encoding
//!
//! All instructions are 32-bit values that follow the RISC-V specification.
//! The emulator handles:
//! - Immediate value sign-extension for I, S, and B formats
//! - Bit-packing for all instruction formats (R, I, S, B, U, J)
//! - Register index encoding in appropriate bit positions
//! - Opcode and function code encoding
//!
//! ## Memory Layout
//!
//! The processor has 1024 bytes of total addressable memory:
//!
//! | Address Range | Size | Purpose | Details |
//! |---------------|------|---------|---------|
//! | 0-511 | 512 bytes | Instruction Memory | Loaded via `load_instructions()`, 256 instructions max |
//! | 512-1023 | 512 bytes | Data Memory | General-purpose data storage, accessible via load/store |
//! | Stack (x2) | Variable | Stack Pointer | Initialized to address 256 |
//!
//! ## Register Conventions
//!
//! | Register | ABI Name | Purpose | Special Properties |
//! |----------|----------|---------|-------------------|
//! | x0 | zero | Always zero | Hard-wired, reads always return 0, writes ignored |
//! | x1 | ra | Return address | Used by `jal` and `jalr` |
//! | x2 | sp | Stack pointer | Initialized to 256 |
//! | x3-x31 | | General-purpose | Free to use |
//!
//! ## Error Handling
//!
//! The emulator uses panics for fatal errors such as:
//! - Out-of-bounds memory access
//! - Invalid register indices
//! - Attempting to write to the zero register (x0)
//! - Instruction decode failures
//!
//! ## Examples
//!
//! See the [`processor`] and [`assembler`] module documentation for more detailed examples.

mod register;
mod instruction;
mod math_utils;
pub mod assembler;
mod immediates;
pub mod processor;
