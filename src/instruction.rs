use crate::instruction::Instruction::{IFormatInstruction, RFormatInstruction, UFormatInstruction};
use crate::register::Register;

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
    const OP_IMM: usize = 0b0010011;
    const OP: usize = 0b0110011;
    const LUI: usize = 0b0110111;
    const AUIPC: usize = 0b0010111;

    pub fn from(bits: usize) -> Option<Instruction> {
        let opcode_mask = 0b1111111;
        let opcode = bits & opcode_mask;
        match opcode {
            Instruction::OP_IMM => Some(Instruction::parse_iformat(bits)),
            Instruction::OP => Some(Instruction::parse_rformat(bits)),
            Instruction::LUI => Some(Instruction::parse_uformat(bits)),
            Instruction::AUIPC => Some(Instruction::parse_uformat(bits)),
            _ => None
        }
    }

    pub fn execute(self, register: &mut Register) {
        match self {
            IFormatInstruction { funct3, rd, rs1, imm } =>
                match funct3 {
                    0b000 => { // Addi
                        let i = register.get(rs1);
                        register.put(rd, i + (imm as usize));
                    },
                    0b001 => { // Slli
                        register.put(rd, register.get(rs1) << imm)
                    },
                    0b010 => { // Slti
                        let i = register.get(rs1) as isize;
                        if i < (imm as isize) {
                            register.put(rd, 1);
                        } else {
                            register.put(rd, 0);
                        }
                    },
                    0b011 => { // Sltiu
                        let i = register.get(rs1);
                        if i < (imm as usize) {
                            register.put(rd, 1);
                        } else {
                            register.put(rd, 0);
                        }
                    },
                    0b100 => { // Xori
                        let i = register.get(rs1);
                        register.put(rd, i ^ (imm as usize));
                    },
                    0b101 => { // Srli and Srai
                        // need to discriminate between srli and srai
                        let discriminator = imm >> 10;
                        match discriminator {
                            0b00 => todo!(),
                            0b01 => todo!(),
                            _ => panic!()
                        }
                    }
                    0b110 => { // Ori
                        let i = register.get(rs1);
                        register.put(rd, i | (imm as usize));
                    },
                    0b111 => { // Andi
                        let i = register.get(rs1);
                        register.put(rd, i & (imm as usize));
                    },
                    _ => return
                },
            RFormatInstruction { funct3, funct7, rs1, rs2, rd } => {
                let funct = funct7 << 3 + funct3;
                match funct {
                    0b0000000000 => { // Add
                        let i = register.get(rs1);
                        let j = register.get(rs2);
                        register.put(rd, i + j);
                    },
                    _ => return
                }
            },
            UFormatInstruction { imm, rd, opcode } =>
                match opcode {
                    Instruction::LUI => {
                        register.put(rd, (imm as usize) << 12);
                    },
                    Instruction::AUIPC => {
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
