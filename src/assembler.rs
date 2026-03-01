//! Assembly language compiler for RISC-V instructions.
//!
//! This module provides the primary interface for converting RISC-V assembly language
//! into machine-executable 32-bit instructions. The assembler is a two-pass compiler that:
//! - Parses assembly instructions and pseudo-instructions
//! - Manages a symbol table for labels and references
//! - Expands pseudo-instructions to their base instruction equivalents
//! - Encodes all instruction formats into 32-bit binary representations
//! - Handles comments and whitespace normalization
//!
//! # Supported Instruction Types
//!
//! - **R-type**: Register-register operations (add, sub, and, or, xor, etc.)
//! - **I-type**: Register-immediate operations (addi, andi, ori, etc.)
//! - **Load (I-type)**: Memory load operations (lw, lh, lb, etc.)
//! - **S-type**: Memory store operations (sw, sh, sb)
//! - **B-type**: Conditional branches (beq, bne, blt, etc.)
//! - **U-type**: Upper immediate loads (lui, auipc)
//! - **J-type**: Unconditional jumps (jal)
//!
//! # Pseudo-Instruction Support
//!
//! The assembler automatically expands pseudo-instructions:
//! - `nop` - No operation
//! - `li rd, imm` - Load immediate
//! - `mv rd, rs` - Move register
//! - `j label` - Jump
//! - And more...
//!
//! # Example
//!
//! ```
//! use risc_v_emulator::assembler::assemble;
//!
//! let assembly = vec![
//!     "addi x5, x0, 42".to_string(),
//!     "add x6, x5, x5".to_string(),
//! ];
//! let code = assemble(assembly);
//! assert_eq!(code.len(), 2);
//! ```

use crate::immediates::BImmediate;
use crate::immediates::{IImmediate, Immediate, JImmediate, SImmediate, UImmediate};
use crate::instruction;
use std::collections::HashMap;

#[derive(Debug)]
struct BOperation {
    instruction: String,
    source1: String,
    source2: String,
    offset: String
}

#[derive(Debug)]
struct IOperation {
    instruction: String,
    source: String,
    immediate: String,
    destination: String
}

#[derive(Debug)]
struct JOperation {
    destination: String,
    immediate: String
}

#[derive(Debug)]
struct ROperation {
    instruction: String,
    source1: String,
    source2: String,
    destination: String
}

#[derive(Debug)]
struct SOperation {
    instruction: String,
    base: String,
    source: String,
    offset: String
}

#[derive(Debug)]
struct UOperation {
    instruction: String,
    destination: String,
    immediate: String
}

trait Operation {
    fn compile(self) -> u32;
}

impl Operation for BOperation {
    fn compile(self) -> u32 {
        println!("[compiling] {:?}", self);
        let rs1: u32 = self.source1.parse().unwrap();
        let rs2: u32 = self.source2.parse().unwrap();
        let imm: i32 = self.offset.parse().unwrap();
        let op: u32 = match self.instruction.as_str() {
            "beq" => instruction::BEQ,
            "bne" => instruction::BNE,
            "blt" => instruction::BLT,
            "bltu" => instruction::BLTU,
            "bge" => instruction::BGE,
            "bgeu" => instruction::BGEU,
            _ => panic!("oops!")
        };

        instruction::BRANCH
            + BImmediate::from(imm as u32).to_instruction_bitmask()
            + (op << 12)
            + (rs1 << 15)
            + (rs2 << 20)
    }
}

impl Operation for IOperation {
    fn compile(self) -> u32 {
        println!("[compiling] {:?}", self);
        let rs1: u32 = self.source.parse().unwrap();
        let imm: i32 = self.immediate.parse().unwrap();
        let rd: u32 = self.destination.parse().unwrap();
        let op: u32 = match self.instruction.as_str() {
            "addi" => instruction::ADDI,
            "slti" => instruction::SLTI,
            "sltiu" => instruction::SLTIU,
            "andi" => instruction::ANDI,
            "ori" => instruction::ORI,
            "xori" => instruction::XORI,
            "slli" => instruction::SLLI,
            "srli" => instruction::SRLI,
            "srai" => instruction::SRAI,
            "jalr" => instruction::JALR,
            "lw" => instruction::LW,
            "lh" => instruction::LH,
            "lhu" => instruction::LHU,
            "lb" => instruction::LB,
            "lbu" => instruction::LBU,
            _ => panic!("oops!")
        };

        let imm = IImmediate::from(imm as u32);

        match self.instruction.as_str() {
            "jalr" => {
                instruction::JALR
                    + (rd << 7)
                    + (rs1 << 15)
                    + imm.to_instruction_bitmask()
            },
            "lw"|"lh"|"lhu"|"lb"|"lbu" => {
                instruction::LOAD
                    + (rd << 7)
                    + (op << 12)
                    + (rs1 << 15)
                    + imm.to_instruction_bitmask()
            },
            _ => {
                instruction::OP_IMM
                    + (rd << 7)
                    + ((op & 0b111) << 12)
                    + (rs1 << 15)
                    + imm.to_instruction_bitmask()
                    + ((op >> 3) << 25)
            }
        }
    }
}

impl Operation for JOperation {
    fn compile(self) -> u32 {
        println!("[compiling] {:?}", self);
        let rd: u32 = self.destination.parse().unwrap();
        let imm: i32 = self.immediate.parse().unwrap();
        let op = instruction::JAL;

        op
            + (rd << 7)
            + JImmediate::from(imm as u32).to_instruction_bitmask()
    }
}

impl Operation for ROperation {
    fn compile(self) -> u32 {
        println!("[compiling] {:?}", self);
        let rs1: u32 = self.source1.parse().unwrap();
        let rs2: u32 = self.source2.parse().unwrap();
        let rd: u32 = self.destination.parse().unwrap();
        let op: u32 = match self.instruction.as_str() {
            "add" => instruction::ADD,
            "slt" => instruction::SLT,
            "sltu" => instruction::SLTU,
            "and" => instruction::AND,
            "or" => instruction::OR,
            "xor" => instruction::XOR,
            "sll" => instruction::SLL,
            "srl" => instruction::SRL,
            "sub" => instruction::SUB,
            "sra" => instruction::SRA,
            _ => panic!("oops!")
        };

        instruction::OP
            + (rd << 7)
            + ((op & 0b111) << 12)
            + (rs1 << 15)
            + (rs2 << 20)
            + ((op >> 3) << 25)
    }
}

impl Operation for SOperation {
    fn compile(self) -> u32 {
        println!("[compiling] {:?}", self);
        let rs1: u32 = self.base.parse().unwrap();
        let rs2: u32 = self.source.parse().unwrap();
        let imm: i32 = self.offset.parse().unwrap();
        let width: u32 = match self.instruction.as_str() {
            "sw" => instruction::SW,
            "sh" => instruction::SH,
            "sb" => instruction::SB,
            _ => panic!("oops!")
        };

        instruction::STORE
            + SImmediate::from(imm as u32).to_instruction_bitmask()
            + (width << 12)
            + (rs1 << 15)
            + (rs2 << 20)
    }
}

impl Operation for UOperation {
    fn compile(self) -> u32 {
        println!("[compiling] {:?}", self);
        let rd: u32 = self.destination.parse().unwrap();
        let imm: u32 = self.immediate.parse().unwrap();
        let op: u32 = match self.instruction.as_str() {
            "lui" => instruction::LUI,
            "auipc" => instruction::AUIPC,
            _ => panic!("oops!")
        };

        op + (rd << 7) + UImmediate::from(imm << 12).to_instruction_bitmask()
    }
}

const B_OPS: &[&str] = &[
    "beq",
    "bne",
    "blt",
    "bltu",
    "bge",
    "bgeu"
];

const I_OPS: &[&str] = &[
    "addi",
    "slti",
    "sltiu",
    "andi",
    "ori",
    "xori",
    "slli",
    "srli",
    "srai",
    "jalr"
];

const I_OPS_LOAD: &[&str] = &[
    "lw",
    "lh",
    "lhu",
    "lb",
    "lbu"
];

const R_OPS: &[&str] = &[
    "add",
    "slt",
    "sltu",
    "and",
    "or",
    "xor",
    "sll",
    "srl",
    "sub",
    "sra",
];

const S_OPS: &[&str] = &[
    "sw",
    "sh",
    "sb"
];

const U_OPS: &[&str] = &[
    "lui",
    "auipc"
];

const DATA_DIRECTIVES: &[&str] = &[
    ".word"
];

fn compile_line(instruction: &str) -> u32 {
    let tokens = instruction.split_whitespace().collect::<Vec<_>>();
    match tokens[0] {
        token if R_OPS.contains(&token) => {
            ROperation {
                instruction: token.to_owned(),
                destination: parse_register(tokens[1]).to_owned(),
                source1: parse_register(tokens[2]).to_owned(),
                source2: parse_register(tokens[3]).to_owned(),
            }.compile()
        },
        token if I_OPS.contains(&token) => {
            IOperation {
                instruction: token.to_owned(),
                destination: parse_register(tokens[1]).to_owned(),
                source: parse_register(tokens[2]).to_owned(),
                immediate: tokens[3].to_owned(),
            }.compile()
        },
        token if I_OPS_LOAD.contains(&token) => {
            let (offset, base) = parse_base_and_offset(tokens[2]);
            IOperation {
                instruction: token.to_owned(),
                destination: parse_register(tokens[1]).to_owned(),
                source: parse_register(base).to_owned(),
                immediate: offset.to_owned(),
            }.compile()
        },
        token if U_OPS.contains(&token) => {
            UOperation {
                instruction: token.to_owned(),
                destination: parse_register(tokens[1]).to_owned(),
                immediate: tokens[2].to_owned()
            }.compile()
        },
        "jal" => {
            JOperation {
                destination: parse_register(tokens[1]).to_owned(),
                immediate: tokens[2].to_owned()
            }.compile()
        },
        token if B_OPS.contains(&token) => {
            BOperation {
                instruction: token.to_owned(),
                source1: parse_register(tokens[1]).to_owned(),
                source2: parse_register(tokens[2]).to_owned(),
                offset: tokens[3].to_owned()
            }.compile()
        },
        token if S_OPS.contains(&token) => {
            let (offset, base) = parse_base_and_offset(tokens[2]);
            SOperation {
                instruction: token.to_owned(),
                source: parse_register(tokens[1]).to_owned(),
                base: parse_register(base).to_owned(),
                offset: offset.to_owned()
            }.compile()
        },
        token if DATA_DIRECTIVES.contains(&token) => {
            parse_data_directive(&tokens)
        }
        _ => panic!("oops! token not found: {}", tokens[0])
    }
}

fn parse_data_directive(tokens: &Vec<&str>) -> u32 {
    match tokens[0] {
        ".word" => tokens[1].parse::<u32>().unwrap(),
        _ => panic!("oops! data directive not found: {}", tokens[0])
    }
}

fn parse_register(token: &str) -> &str {
    match token.trim_end_matches(',') {
        t if t.starts_with('x') => t.trim_start_matches('x'),
        "zero" => "0",
        "ra" => "1",
        "sp" => "2",
        "gp" => "3",
        "tp" => "4",
        "t0" => "5",
        "t1" => "6",
        "t2" => "7",
        "s0" | "fp" => "8",
        "s1" => "9",
        "a0" => "10",
        "a1" => "11",
        "a2" => "12",
        "a3" => "13",
        "a4" => "14",
        "a5" => "15",
        "a6" => "16",
        "a7" => "17",
        "s2" => "18",
        "s3" => "19",
        "s4" => "20",
        "s5" => "21",
        "s6" => "22",
        "s7" => "23",
        "s8" => "24",
        "s9" => "25",
        "s10" => "26",
        "s11" => "27",
        "t3" => "28",
        "t4" => "29",
        "t5" => "30",
        "t6" => "31",
        t => t
    }
}

fn parse_base_and_offset(token: &str) -> (&str, &str) {
    token.strip_suffix(')')
        .and_then(|c| c.split_once('('))
        .unwrap()
}

fn pseudo_to_base_instructions(instruction: &str, symbol_table: &HashMap<&str, usize>, instruction_location: usize) -> Option<Vec<String>> {
    let tokens = instruction.split_whitespace()
        .map(|t| t.trim_end_matches(','))
        .collect::<Vec<_>>();
    match tokens[0] {
        "nop" => Some(vec![
            String::from("addi x0, x0, 0")
        ]),
        "li" => Some(vec![
            format!("addi {rd}, x0, {imm}", rd=tokens[1], imm=tokens[2])
        ]),
        "mv" => Some(vec![
            format!("addi {rd}, {rs}, 0", rd=tokens[1], rs=tokens[2])
        ]),
        "not" => Some(vec![
            format!("xori {rd}, {rs}, -1", rd=tokens[1], rs=tokens[2])
        ]),
        "neg" => Some(vec![
            format!("sub {rd}, x0, {rs}", rd=tokens[1], rs=tokens[2])
        ]),
        "seqz" => Some(vec![
            format!("sltiu {rd}, {rs}, 1", rd=tokens[1], rs=tokens[2])
        ]),
        "snez" => Some(vec![
            format!("sltu {rd}, x0, {rs}", rd=tokens[1], rs=tokens[2])
        ]),
        "sltz" => Some(vec![
            format!("slt {rd}, {rs}, x0", rd=tokens[1], rs=tokens[2])
        ]),
        "sqtz" => Some(vec![
            format!("slt {rd}, x0, {rs}", rd=tokens[1], rs=tokens[2])
        ]),
        "beqz" => Some(vec![
            if let Some(addr) = symbol_table.get(tokens[2]) {
                let offset = get_offset(*addr as i32, instruction_location as i32);
                format!("beq {rs}, x0, {offset}", rs = tokens[1], offset = offset)
            } else {
                format!("beq {rs}, x0, {offset}", rs = tokens[1], offset = tokens[2])
            }
        ]),
        "bnez" => Some(vec![
            if let Some(addr) = symbol_table.get(tokens[2]) {
                let offset = get_offset(*addr as i32, instruction_location as i32);
                format!("bne {rs}, x0, {offset}", rs=tokens[1], offset=offset)
            } else {
                format!("bne {rs}, x0, {offset}", rs=tokens[1], offset=tokens[2])
            }
        ]),
        "bgt" => Some(vec![
            if let Some(addr) = symbol_table.get(tokens[3]) {
                let offset = get_offset(*addr as i32, instruction_location as i32);
                format!("blt {rt}, {rs}, {offset}", rt = tokens[2], rs = tokens[1], offset = offset)
            } else {
                format!("blt {rt}, {rs}, {offset}", rt = tokens[2], rs = tokens[1], offset = tokens[3])
            }
        ]),
        "ble" => Some(vec![
            if let Some(addr) = symbol_table.get(tokens[3]) {
                let offset = get_offset(*addr as i32, instruction_location as i32);
                format!("bge {rt}, {rs}, {offset}", rt = tokens[2], rs = tokens[1], offset = offset)
            } else {
                format!("bge {rt}, {rs}, {offset}", rt=tokens[2], rs=tokens[1], offset=tokens[3])
            }
        ]),
        "j" => {
            if let Some(addr) = symbol_table.get(tokens[1]) {
                let offset = get_offset(*addr as i32, instruction_location as i32);
                Some(vec![
                    format!("jal x0, {offset}", offset=offset)
                ])
            } else {
                Some(vec![
                    format!("jal x0, {offset}", offset=tokens[1])
                ])
            }
        },
        "ret" => Some(vec![
            String::from("jalr x0, x1, 0")
        ]),
        "call" => {
            if let Some(addr) = symbol_table.get(tokens[1]) {
                let offset = get_offset(*addr as i32, instruction_location as i32);
                Some(vec![
                    format!("jal x1, {offset}", offset=offset)
                ])
            } else {
                let msb = tokens[1].parse::<u32>().unwrap() >> 12;
                let lsb = tokens[1].parse::<u32>().unwrap() & 0b111111111111;
                Some(vec![
                    format!("auipc x6, {offset}", offset = msb),
                    format!("jalr x1, x6, {offset}", offset = (lsb))
                ])
            }
        },
        token if I_OPS_LOAD.contains(&token) => {
            if let Some(addr) = symbol_table.get(tokens[2]) {
                Some(vec![
                    format!("addi {rd}, x0, {imm}", rd=tokens[1], imm=addr),
                    format!("{op} {rd}, 0({rd})", op=tokens[0], rd=tokens[1])
                ])
            } else {
                None
            }
        },
        token if B_OPS.contains(&token) => {
            if let Some(addr) = symbol_table.get(tokens[3]) {
                let offset = get_offset(*addr as i32, instruction_location as i32);
                Some(vec![
                    format!("{op} {rt}, {rs}, {offset}", op=tokens[0], rt=tokens[1], rs=tokens[2], offset=offset)
                ])
            } else {
                None
            }
        }
        _ => None
    }
}

fn get_offset(addr: i32, instruction_location: i32) -> i32 {
    addr - (instruction_location + 4)
}

/// Assembles RISC-V assembly code into executable 32-bit machine instructions.
///
/// This is the main public API for the assembler module. It takes a vector of assembly
/// instruction strings and performs a complete two-pass assembly process:
/// 1. **First Pass**: Parse instructions, collect labels into symbol table
/// 2. **Second Pass**: Expand pseudo-instructions and compile to machine code
///
/// The function returns a vector of 32-bit instruction words ready for execution
/// by the [`crate::processor::Processor`].
///
/// # Arguments
///
/// * `instructions` - A vector of assembly instruction strings
///   - Each string is one instruction or label
///   - Can include inline comments (lines or parts starting with `#`)
///   - Can include whitespace and empty lines
///
/// # Returns
///
/// A `Vec<u32>` containing the assembled machine code where:
/// - Each element is a 32-bit instruction word
/// - Instructions are in program order (can be loaded sequentially)
/// - The vector is ready for loading into a [`crate::processor::Processor`]
///
/// # Supported Instruction Types
///
/// ## Base Instructions (All RISC-V I-type, R-type, S-type, B-type, U-type, J-type)
///
/// **R-type (Register-Register)**:
/// - Arithmetic: `add rd, rs1, rs2`, `sub rd, rs1, rs2`, `and rd, rs1, rs2`,
///   `or rd, rs1, rs2`, `xor rd, rs1, rs2`, `sll rd, rs1, rs2`, `srl rd, rs1, rs2`,
///   `sra rd, rs1, rs2`
/// - Comparison: `slt rd, rs1, rs2`, `sltu rd, rs1, rs2`
///
/// **I-type (Register-Immediate)**:
/// - Arithmetic: `addi rd, rs1, imm`, `andi rd, rs1, imm`, `ori rd, rs1, imm`,
///   `xori rd, rs1, imm`, `slli rd, rs1, imm`, `srli rd, rs1, imm`, `srai rd, rs1, imm`
/// - Comparison: `slti rd, rs1, imm`, `sltiu rd, rs1, imm`
/// - Jump: `jalr rd, rs1, imm`
///
/// **Load (I-type)**:
/// - `lw rd, offset(rs1)` - Load word
/// - `lh rd, offset(rs1)` - Load half-word (sign-extended)
/// - `lhu rd, offset(rs1)` - Load half-word unsigned (zero-extended)
/// - `lb rd, offset(rs1)` - Load byte (sign-extended)
/// - `lbu rd, offset(rs1)` - Load byte unsigned (zero-extended)
///
/// **S-type (Store)**:
/// - `sw rs2, offset(rs1)` - Store word
/// - `sh rs2, offset(rs1)` - Store half-word
/// - `sb rs2, offset(rs1)` - Store byte
///
/// **B-type (Branch)**:
/// - `beq rs1, rs2, label` - Branch if equal
/// - `bne rs1, rs2, label` - Branch if not equal
/// - `blt rs1, rs2, label` - Branch if less than
/// - `bge rs1, rs2, label` - Branch if greater or equal
/// - `bltu rs1, rs2, label` - Branch if less than (unsigned)
/// - `bgeu rs1, rs2, label` - Branch if greater or equal (unsigned)
///
/// **U-type (Upper Immediate)**:
/// - `lui rd, imm` - Load upper immediate
/// - `auipc rd, imm` - Add upper immediate to PC
///
/// **J-type (Jump)**:
/// - `jal rd, label` - Jump and link
///
/// # Pseudo-Instructions (Automatically Expanded)
///
/// **Control Flow**:
/// - `nop` - No operation (expands to `addi x0, x0, 0`)
/// - `j label` - Jump unconditionally
/// - `ret` - Return from function
/// - `call label` - Call a function
///
/// **Data Movement**:
/// - `li rd, imm` - Load immediate (handles 32-bit immediates)
/// - `mv rd, rs` - Move register (copy)
///
/// **Logical Operations**:
/// - `not rd, rs` - Logical NOT
/// - `neg rd, rs` - Negate
///
/// **Zero Comparisons**:
/// - `seqz rd, rs` - Set if equal to zero
/// - `snez rd, rs` - Set if not equal to zero
/// - `sltz rd, rs` - Set if less than zero
/// - `sgtz rd, rs` - Set if greater than zero
///
/// **Branch Pseudo-Instructions**:
/// - `beqz rs, label` - Branch if equal to zero
/// - `bnez rs, label` - Branch if not equal to zero
/// - `bgt rs1, rs2, label` - Branch if greater than
/// - `ble rs1, rs2, label` - Branch if less or equal
///
/// # Labels and Symbols
///
/// Labels are defined by ending a line with `:` (e.g., `loop:`, `start:`):
/// - Labels are automatically collected into a symbol table
/// - Label addresses are calculated during the first pass
/// - Labels can be referenced in branch and jump instructions
/// - Forward references (jumping to labels later in code) are supported
/// - A special `start` label marks the program entry point
///
/// # Comments
///
/// - Lines or portions of lines starting with `#` are comments
/// - Comments are ignored during assembly
/// - Inline comments are supported: `addi x5, x0, 42 # Load 42 into x5`
///
/// # Whitespace Handling
///
/// - Leading and trailing whitespace is automatically stripped
/// - Empty lines and lines with only comments are ignored
/// - Multiple spaces between operands are collapsed
///
/// # Register Names
///
/// Registers can be referenced by:
/// - **Number**: `x0`, `x1`, `x2`, ..., `x31`
/// - **ABI Name**: `zero`, `ra`, `sp`, `gp`, `tp`, `t0`, `s0`, `a0`, etc.
///
/// # Immediate Values
///
/// Immediates can be specified as:
/// - **Decimal**: `42`, `-100`, `2048`
/// - **Hexadecimal**: `0xFF`, `0x1000`
/// - **Binary**: `0b1010`
/// - **Labels**: In branch/jump instructions (resolved to addresses)
///
/// # Output and Side Effects
///
/// The function prints debug information to stdout:
/// - `[compiling] Symbol table:` - Lists all labels and their addresses
/// - `[compiling] <instruction details>` - Details of each compiled instruction
/// - `<32-bit binary>` - The binary representation of each instruction
///
/// This output is useful for debugging assembly but can be verbose for large programs.
///
/// # Panics
///
/// The function may panic in these situations:
/// - Invalid register names or indices
/// - Malformed instruction syntax
/// - Invalid operand values or types
/// - Label references that cannot be resolved
/// - Immediate values out of valid ranges for their instruction type
///
/// # Complexity
///
/// - Time: O(n) where n is the number of instructions
/// - Space: O(n) for the symbol table and output vector
///
/// # Example: Simple Arithmetic Program
///
/// ```
/// use risc_v_emulator::assembler::assemble;
///
/// let assembly = vec![
///     "addi x5, x0, 10".to_string(),   // x5 = 10
///     "addi x6, x0, 20".to_string(),   // x6 = 20
///     "add x7, x5, x6".to_string(),    // x7 = 30
/// ];
///
/// let executable = assemble(assembly);
/// assert_eq!(executable.len(), 3);    // Three instructions compiled
/// ```
///
/// # Example: Program with Labels
///
/// ```no_run
/// use risc_v_emulator::assembler::assemble;
///
/// let assembly = vec![
///     "addi x5, x0, 5".to_string(),    // Counter = 5
///     "loop:".to_string(),
///     "addi x5, x5, -1".to_string(),   // Decrement counter
///     "bne x5, x0, loop".to_string(),  // Jump back if not zero
/// ];
///
/// let executable = assemble(assembly);
/// ```
///
/// # Example: Pseudo-Instructions
///
/// ```no_run
/// use risc_v_emulator::assembler::assemble;
///
/// let assembly = vec![
///     "li x5, 0x12345678".to_string(),  // Load 32-bit immediate
///     "mv x6, x5".to_string(),          // Copy register
///     "nop".to_string(),                // No-op
/// ];
///
/// let executable = assemble(assembly);
/// ```
///
/// # See Also
///
/// - [`crate::processor::Processor::load_instructions`] - Load assembled code
/// - [`crate::processor::Processor::execute_instructions`] - Execute assembled code
pub fn assemble(instructions: Vec<String>) -> Vec<u32> {
    let mut symbol_table = HashMap::new();
    let mut trimmed_instructions = vec!();
    let mut active_location_counter = 0;

    let re = regex::Regex::new("#.*$").unwrap();
    for instruction in instructions.iter() {

        let (instruction, _comments): (&str, &str) =
            if let Some(captures) = re.captures(&instruction) {
                let x = captures.get(0).unwrap();
                instruction.split_at(x.start())
            } else {
                (instruction, "")
            };

        let instruction = instruction.trim();
        if instruction.ends_with(":") {
            let symbol = instruction.strip_suffix(":").unwrap();
            symbol_table.insert(symbol, active_location_counter);
        } else if !instruction.is_empty() {
            trimmed_instructions.push(instruction);
            active_location_counter += 4;
        }
    }

    println!("[compiling] Symbol table: ");
    symbol_table.iter()
        .for_each(|(key, value)| {
            println!("  {:08x} {}", value, key);
        });

    let start;
    if let Some(addr) = symbol_table.get("start") {
        start = format!("j {}", addr);
        trimmed_instructions.insert(0, start.as_str());
        symbol_table = symbol_table
            .iter()
            .map(|(key, value)| {
                (*key, value + 4)
            })
            .collect()
    }

    trimmed_instructions
        .iter()
        .enumerate()
        .flat_map(|(location, instruction)| {
            pseudo_to_base_instructions(instruction, &symbol_table, location * 4)
                .unwrap_or(vec![instruction.to_string()])
        })
        .map(|instruction: String| {
            println!("[compiling] Instruction: '{}'", instruction);
            let binary = compile_line(&instruction);
            println!("[compiling] Output: '{:0>32b}'", binary);
            binary
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::assembler::{assemble, compile_line};

    #[test]
    fn test_compile_add() {
        let instruction = "add x5, x0, x1";

        let op = compile_line(instruction);

        assert_eq!(op, 0b0000000_00001_00000_000_00101_0110011)
    }

    #[test]
    fn test_compile_slt() {
        let instruction = "slt x5, x0, x1";

        let op = compile_line(instruction);

        assert_eq!(op, 0b0000000_00001_00000_010_00101_0110011)
    }

    #[test]
    fn test_compile_sra() {
        let instruction = "sra x5, x0, x1";

        let op = compile_line(instruction);

        assert_eq!(op, 0b0100000_00001_00000_101_00101_0110011)
    }

    #[test]
    fn test_compile_addi() {
        let instruction = "addi x5, x4, 20";

        let op = compile_line(instruction);

        assert_eq!(op, 0b000000010100_00100_000_00101_0010011)
    }

    #[test]
    fn test_compile_slti() {
        let instruction = "slti x5, x4, 20";

        let op = compile_line(instruction);

        assert_eq!(op, 0b000000010100_00100_010_00101_0010011)
    }

    #[test]
    fn test_compile_srai() {
        let instruction = "srai x5, x0, 20";

        let op = compile_line(instruction);

        assert_eq!(op, 0b0100000_10100_00000_101_00101_0010011)
    }

    #[test]
    fn test_compile_lui() {
        let instruction = "lui x5, 1234";

        let op = compile_line(instruction);
        println!("{:0>32b}", op);
        assert_eq!(op, 0b00000000010011010010_00101_0110111)
    }

    #[test]
    fn test_compile_jal() {
        let instruction = "jal x5, 1234";

        let op = compile_line(instruction);

        println!("{:0>32b}", op);
        assert_eq!(op, 0b01001101001000000000001011101111)
    }

    #[test]
    fn test_compile_jalr() {
        let instruction = "jalr x5, x3, 1234";

        let op = compile_line(instruction);

        assert_eq!(op, 0b010011010010_00011_000_00101_1100111)
    }

    #[test]
    fn test_compile_beq() {
        let instruction = "beq x5, x3, 1234";

        let op = compile_line(instruction);

        assert_eq!(op, 0b0_100110_00011_00101_000_1001_0_1100011)
    }

    #[test]
    fn test_compile_bltu() {
        let instruction = "bltu x5, x3, 1234";

        let op = compile_line(instruction);

        assert_eq!(op, 0b0_100110_00011_00101_110_1001_0_1100011)
    }

    #[test]
    fn test_compile_lw() {
        let instruction = "lw t2, 0(t3)";

        let op = compile_line(instruction);

        assert_eq!(op, 0b000000000000_11100_010_00111_0000011)
    }

    #[test]
    fn test_compile_lbu() {
        let instruction = "lbu t2, 0(t3)";

        let op = compile_line(instruction);

        assert_eq!(op, 0b000000000000_11100_100_00111_0000011)
    }

    #[test]
    fn test_compile_sw() {
        let instruction = "sw t2, 0(t3)";

        let op = compile_line(instruction);

        assert_eq!(op, 0b0000000_00111_11100_010_00000_0100011)
    }

    #[test]
    fn test_compile_beqz() {
        let instruction = "beqz t2, 6".to_string();

        let ops = assemble(vec![instruction]);

        assert_eq!(ops, vec![0b0_000000_00000_00111_000_0011_0_1100011])
    }

    #[test]
    fn test_compile_call() {
        let instruction = "call 123456789".to_string();

        let ops = assemble(vec![instruction]);

        assert_eq!(ops, vec![
            0b00000111010110111100_00110_0010111,
            0b110100010101_00110_000_00001_1100111
        ])
    }

    #[test]
    fn test_variable_label() {
        let instructions = vec![
            "x:".to_string(),
            ".word 10".to_string(),
            "lw a0, x".to_string()
        ];

        let ops = assemble(instructions);

        assert_eq!(ops, vec![
            0b00000000000000000000000000001010,
            0b000000000000_00000_000_01010_0010011,
            0b000000000000_01010_010_01010_0000011
        ])
    }

}
