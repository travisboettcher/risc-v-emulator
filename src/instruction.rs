use crate::instruction::Instruction::{IFormatInstruction, RFormatInstruction, UFormatInstruction};
use crate::register::Register;
use crate::math_utils::MixedIntegerOps;

/// opcodes
const OP_IMM: u32 = 0b0010011;
const OP: u32     = 0b0110011;
const LUI: u32    = 0b0110111;
const AUIPC: u32  = 0b0010111;
const FENCE: u32  = 0b0001111;

/// functions
const ADDI: u32  = 0b000;
const SLLI: u32  = 0b001;
const SLTI: u32  = 0b010;
const SLTIU: u32 = 0b011;
const XORI: u32  = 0b100;
const SRLI: u32  = 0b101;
const ORI: u32   = 0b110;
const ANDI: u32  = 0b111;

const ADD: u32  = 0b0000000000;
const SUB: u32  = 0b0100000000;
const SLL: u32  = 0b0000000001;
const SLT: u32  = 0b0000000010;
const SLTU: u32 = 0b0000000011;
const XOR: u32  = 0b0000000100;
const SRL: u32  = 0b0000000101;
const SRA: u32  = 0b0100000101;
const OR: u32   = 0b0000000110;
const AND: u32  = 0b0000000111;

#[derive(Debug)]
pub enum Instruction {
    IFormatInstruction {
        imm: i16,
        rs1: usize,
        funct3: u32,
        rd: usize
    },
    RFormatInstruction {
        rd: usize,
        funct3: u32,
        rs1: usize,
        rs2: usize,
        funct7: u32
    },
    UFormatInstruction {
        imm: i32,
        rd: usize,
        opcode: u32
    }
}

impl Instruction {
    pub fn from(bits: u32) -> Option<Instruction> {
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
                        register.put(rd, MixedIntegerOps::wrapping_add_signed(i, imm as i32));
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
                        if i < (imm as u32) {
                            register.put(rd, 1);
                        } else {
                            register.put(rd, 0);
                        }
                    },
                    XORI => { // Xori
                        let i = register.get(rs1);
                        register.put(rd, i ^ (imm as u32));
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
                                register.put(rd, (i >> imm) as u32);
                            },
                            _ => panic!()
                        }
                    }
                    ORI => { // Ori
                        let i = register.get(rs1);
                        register.put(rd, i | (imm as u32));
                    },
                    ANDI => { // Andi
                        let i = register.get(rs1);
                        register.put(rd, i & (imm as u32));
                    },
                    _ => return
                },
            RFormatInstruction { funct3, funct7, rs1, rs2, rd } => {
                let funct = (funct7 << 3) + funct3;
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
                        register.put(rd, (i >> j) as u32);
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
                        register.put(rd, (imm as u32) << 12);
                    },
                    AUIPC => {
                        let u_immediate = (imm as u32) << 12;
                        register.put(rd, register.pc() + u_immediate);
                    },
                    _ => return
                }
        }
    }

    fn parse_iformat(bits: u32) -> Instruction {
        let rd = (bits >> 7 & 0b11111) as usize;
        let funct3 = bits >> 12 & 0b111;
        let rs1 = (bits >> 15 & 0b11111) as usize;
        let imm = (bits >> 20) as i16;
        IFormatInstruction {
            imm,
            rs1,
            funct3,
            rd
        }
    }

    fn parse_rformat(bits: u32) -> Instruction {
        let rd = (bits >> 7 & 0b11111) as usize;
        let funct3 = bits >> 12 & 0b111;
        let rs1 = (bits >> 15 & 0b11111) as usize;
        let rs2 = (bits >> 20 & 0b11111) as usize;
        let funct7 = bits >> 25;
        RFormatInstruction {
            rs1,
            rs2,
            funct3,
            funct7,
            rd
        }
    }

    fn parse_uformat(bits: u32) -> Instruction {
        let opcode = bits & 0b1111111;
        let rd = (bits >> 7 & 0b11111) as usize;
        let imm = (bits >> 12) as i32;
        UFormatInstruction {
            imm,
            rd,
            opcode
        }
    }
}

#[test]
fn test_add() {
    let mut register = Register::new();
    register.put(4, 0x7fffffff);
    register.put(24, 0x1);

    let instruction = RFormatInstruction {
        rd: 25,
        funct3: 0b000,
        rs1: 4,
        rs2: 24,
        funct7: 0b0000000
    };
    instruction.execute(&mut register);

    assert_eq!(register.get(25), 0x80000000);
}

#[test]
fn test_addi() {
    let mut register = Register::new();
    register.put(20, 0x20000000);
    
    let instruction = IFormatInstruction {
        imm: -0x800,
        rs1: 20,
        funct3: ADDI,
        rd: 7
    };
    instruction.execute(&mut register);

    assert_eq!(register.get(7), 0x1ffff800);
}

#[test]
fn test_and() {
    let mut register = Register::new();
    register.put(10, 0x3);
    register.put(11, 0x55555556);

    let instruction = RFormatInstruction {
        rd: 12,
        funct3: 0b111,
        rs1: 10,
        rs2: 11,
        funct7: 0b0000000
    };
    instruction.execute(&mut register);

    assert_eq!(register.get(12), 0x2);
}

#[test]
fn test_andi() {
    let mut register = Register::new();
    register.put(10, 0x55555555);

    let instruction = IFormatInstruction {
        imm: 0x334,
        rs1: 10,
        funct3: ANDI,
        rd: 11
    };
    instruction.execute(&mut register);

    assert_eq!(register.get(11), 0x114);
}

#[test]
fn test_auipc() {
    let mut register = Register::new();

    let instruction = UFormatInstruction {
        imm: 0x100,
        rd: 10,
        opcode: AUIPC
    };
    instruction.execute(&mut register);

    assert_eq!(register.get(10), 0x100000);
}

#[test]
fn test_lui() {
    let mut register = Register::new();

    let instruction = UFormatInstruction {
        imm: 0x3,
        rd: 13,
        opcode: LUI
    };
    instruction.execute(&mut register);

    assert_eq!(register.get(13), 0x3000);
}

#[test]
fn test_or() {
    let mut register = Register::new();
    register.put(8, 0x100000);
    register.put(26, 0x10);

    let instruction = RFormatInstruction {
        rd: 26,
        funct3: 0b110,
        rs1: 8,
        rs2: 26,
        funct7: 0b0000000
    };
    instruction.execute(&mut register);

    assert_eq!(register.get(26), 0x100010);
}

#[test]
fn test_ori() {
    let mut register = Register::new();
    register.put(17, 0x33333334);

    let instruction = IFormatInstruction {
        imm: 0x7ff,
        rs1: 17,
        funct3: ORI,
        rd: 8
    };
    instruction.execute(&mut register);

    assert_eq!(register.get(8), 0x333337ff);
}

#[test]
fn test_sll() {
    let mut register = Register::new();
    register.put(12, 0x7fffffff);
    register.put(26, 0x15);

    let instruction = RFormatInstruction {
        rd: 16,
        funct3: 0b001,
        rs1: 12,
        rs2: 26,
        funct7: 0b0000000
    };
    instruction.execute(&mut register);

    assert_eq!(register.get(16), 0xffe00000);
}

#[test]
fn test_slli() {
    let mut register = Register::new();
    register.put(26, 0x66666666);

    let instruction = IFormatInstruction {
        imm: 0xf,
        rs1: 26,
        funct3: SLLI,
        rd: 26
    };
    instruction.execute(&mut register);

    assert_eq!(register.get(26), 0x33330000);
}