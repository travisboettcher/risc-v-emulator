use crate::instruction::Instruction::{IFormatInstruction, RFormatInstruction, UFormatInstruction};
use crate::register::Register;
use crate::math_utils::MixedIntegerOps;

/// opcodes
const OP_IMM: usize = 0b0010011;
const OP: usize     = 0b0110011;
const LUI: usize    = 0b0110111;
const AUIPC: usize  = 0b0010111;
const FENCE: usize  = 0b0001111;

/// functions
const ADDI: usize  = 0b000;
const SLLI: usize  = 0b001;
const SLTI: usize  = 0b010;
const SLTIU: usize = 0b011;
const XORI: usize  = 0b100;
const SRLI: usize  = 0b101;
const ORI: usize   = 0b110;
const ANDI: usize  = 0b111;

const ADD: usize  = 0b0000000000;
const SUB: usize  = 0b0100000000;
const SLL: usize  = 0b0000000001;
const SLT: usize  = 0b0000000010;
const SLTU: usize = 0b0000000011;
const XOR: usize  = 0b0000000100;
const SRL: usize  = 0b0000000101;
const SRA: usize  = 0b0100000101;
const OR: usize   = 0b0000000110;
const AND: usize  = 0b0000000111;

#[derive(Debug)]
pub enum Instruction {
    IFormatInstruction {
        imm: i16,
        rs1: usize,
        funct3: usize,
        rd: usize
    },
    RFormatInstruction {
        rd: usize,
        funct3: usize,
        rs1: usize,
        rs2: usize,
        funct7: usize
    },
    UFormatInstruction {
        imm: i32,
        rd: usize,
        opcode: usize
    }
}

impl Instruction {
    pub fn from(bits: usize) -> Option<Instruction> {
        let opcode_mask = 0b1111111;
        let opcode = bits & opcode_mask;
        match opcode {
            OP_IMM => Some(Instruction::parse_iformat(bits)),
            OP => Some(Instruction::parse_rformat(bits)),
            LUI => Some(Instruction::parse_uformat(bits)),
            AUIPC => Some(Instruction::parse_uformat(bits)),
            FENCE => todo!(),
            _ => None
        }
    }

    pub fn execute(self, register: &mut Register) {
        match self {
            IFormatInstruction { funct3, rd, rs1, imm } =>
                match funct3 {
                    ADDI => { // Addi
                        let i = register.get(rs1);
                        register.put(rd, MixedIntegerOps::wrapping_add_signed(i, imm as isize));
                    },
                    SLLI => { // Slli
                        register.put(rd, register.get(rs1) << imm)
                    },
                    SLTI => { // Slti
                        let i = register.get(rs1) as isize;
                        if i < (imm as isize) {
                            register.put(rd, 1);
                        } else {
                            register.put(rd, 0);
                        }
                    },
                    SLTIU => { // Sltiu
                        let i = register.get(rs1);
                        if i < (imm as usize) {
                            register.put(rd, 1);
                        } else {
                            register.put(rd, 0);
                        }
                    },
                    XORI => { // Xori
                        let i = register.get(rs1);
                        register.put(rd, i ^ (imm as usize));
                    },
                    SRLI => { // Srli and Srai
                        // need to discriminate between srli and srai
                        let discriminator = imm >> 10;
                        match discriminator {
                            0b00 => {
                                let i = register.get(rs1);
                                register.put(rd, i >> imm);
                            },
                            0b01 => {
                                let i = register.get(rs1) as isize;
                                register.put(rd, (i >> imm) as usize);
                            },
                            _ => panic!()
                        }
                    }
                    ORI => { // Ori
                        let i = register.get(rs1);
                        register.put(rd, i | (imm as usize));
                    },
                    ANDI => { // Andi
                        let i = register.get(rs1);
                        register.put(rd, i & (imm as usize));
                    },
                    _ => return
                },
            RFormatInstruction { funct3, funct7, rs1, rs2, rd } => {
                let funct = funct7 << 3 + funct3;
                match funct {
                    ADD => { // Add
                        let i = register.get(rs1);
                        let j = register.get(rs2);
                        register.put(rd, i + j);
                    },
                    SUB => {
                        let i = register.get(rs1);
                        let j = register.get(rs2);
                        register.put(rd, i - j);
                    },
                    SLL => {
                        let i = register.get(rs1);
                        let j = register.get(rs2) & 0b11111;
                        register.put(rd, i << j)
                    },
                    SLT => {
                        let i = register.get(rs1) as isize;
                        let j = register.get(rs2) as isize;
                        if i < j {
                            register.put(rd, 1);
                        } else {
                            register.put(rd, 0);
                        }
                    },
                    SLTU => {
                        let i = register.get(rs1);
                        let j = register.get(rs2);
                        if i < j {
                            register.put(rd, 1);
                        } else {
                            register.put(rd, 0);
                        }
                    },
                    XOR => {
                        let i = register.get(rs1);
                        let j = register.get(rs2);
                        register.put(rd, i ^ j);
                    },
                    SRL => {
                        let i = register.get(rs1);
                        let j = register.get(rs2) & 0b11111;
                        register.put(rd, i >> j);
                    },
                    SRA => {
                        let i = register.get(rs1) as isize;
                        let j = register.get(rs2) & 0b11111;
                        register.put(rd, (i >> j) as usize);
                    },
                    OR => {
                        let i = register.get(rs1);
                        let j = register.get(rs2);
                        register.put(rd, i | j);
                    },
                    AND => {
                        let i = register.get(rs1);
                        let j = register.get(rs2);
                        println!("i: {}, j: {}, i & j: {}", i, j, i&j);
                        register.put(rd, i & j);
                    }
                    _ => return
                }
            },
            UFormatInstruction { imm, rd, opcode } =>
                match opcode {
                    LUI => {
                        register.put(rd, (imm as usize) << 12);
                    },
                    AUIPC => {
                        let u_immediate = (imm as usize) << 12;
                        register.put(rd, register.pc() + u_immediate);
                    },
                    _ => return
                }
        }
    }

    fn parse_iformat(bits: usize) -> Instruction {
        let rd = bits >> 7 & 0b11111;
        let funct3 = bits >> 12 & 0b111;
        let rs1 = bits >> 15 & 0b11111;
        let imm = (bits >> 20) as i16;
        IFormatInstruction {
            imm,
            rs1,
            funct3,
            rd
        }
    }

    fn parse_rformat(bits: usize) -> Instruction {
        let rd = bits >> 7 & 0b11111;
        let funct3 = bits >> 12 & 0b111;
        let rs1 = bits >> 15 & 0b11111;
        let rs2 = bits >> 20 & 0b11111;
        let funct7 = bits >> 25;
        RFormatInstruction {
            rs1,
            rs2,
            funct3,
            funct7,
            rd
        }
    }

    fn parse_uformat(bits: usize) -> Instruction {
        let opcode = bits & 0b1111111;
        let rd = bits >> 7 & 0b11111;
        let imm = (bits >> 12) as i32;
        UFormatInstruction {
            imm,
            rd,
            opcode
        }
    }
}
