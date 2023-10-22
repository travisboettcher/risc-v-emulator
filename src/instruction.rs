use Instruction::{
    BFormatInstruction,
    IFormatInstruction,
    RFormatInstruction,
    UFormatInstruction,
    JFormatInstruction,
    SFormatInstruction
};
use crate::immediates::{BImmediate, IImmediate, Immediate, JImmediate, SImmediate, UImmediate};
use crate::register::Register;
use crate::math_utils::MixedIntegerOps;

/// opcodes
pub const OP_IMM: u32 = 0b0010011;
pub const OP: u32     = 0b0110011;
pub const LUI: u32    = 0b0110111;
pub const AUIPC: u32  = 0b0010111;
pub const FENCE: u32  = 0b0001111;
pub const JALR: u32   = 0b1100111;
pub const JAL: u32    = 0b1101111;
pub const BRANCH: u32 = 0b1100011;
pub const LOAD: u32   = 0b0000011;
pub const STORE: u32  = 0b0100011;

/// functions
pub const ADDI: u32  = 0b0000000000;
pub const SLLI: u32  = 0b0000000001;
pub const SLTI: u32  = 0b0000000010;
pub const SLTIU: u32 = 0b0000000011;
pub const XORI: u32  = 0b0000000100;
pub const SRLI: u32  = 0b0000000101;
pub const SRAI: u32  = 0b0100000101;
pub const ORI: u32   = 0b0000000110;
pub const ANDI: u32  = 0b0000000111;

pub const ADD: u32  = 0b0000000000;
pub const SUB: u32  = 0b0100000000;
pub const SLL: u32  = 0b0000000001;
pub const SLT: u32  = 0b0000000010;
pub const SLTU: u32 = 0b0000000011;
pub const XOR: u32  = 0b0000000100;
pub const SRL: u32  = 0b0000000101;
pub const SRA: u32  = 0b0100000101;
pub const OR: u32   = 0b0000000110;
pub const AND: u32  = 0b0000000111;

pub const BEQ: u32  = 0b000;
pub const BNE: u32  = 0b001;
pub const BLT: u32  = 0b100;
pub const BGE: u32  = 0b101;
pub const BLTU: u32 = 0b110;
pub const BGEU: u32 = 0b111;

pub const LB: u32  = 0b000;
pub const LH: u32  = 0b001;
pub const LW: u32  = 0b010;
pub const LBU: u32 = 0b100;
pub const LHU: u32 = 0b101;

pub const SB: u32 = 0b000;
pub const SH: u32 = 0b001;
pub const SW: u32 = 0b010;

type Memory = [u32; 1024];

#[derive(Debug)]
pub enum Instruction {
    IFormatInstruction {
        imm: i16,
        rs1: usize,
        funct3: u32,
        rd: usize,
        opcode: u32
    },
    JFormatInstruction {
        imm: i32,
        rd: usize,
        opcode: u32
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
    },
    BFormatInstruction {
        imm: i32,
        rs1: usize,
        rs2: usize,
        funct3: u32
    },
    SFormatInstruction {
        imm: i32,
        rs1: usize,
        rs2: usize,
        funct3: u32
    }
}

// Implement SCALL/SBREAK/CSRR* with a single SYSTEM instruction that always traps
// Implement FENCE and FENCE.I as NOPs

impl Instruction {
    pub fn from(bits: u32) -> Option<Instruction> {
        let opcode_mask = 0b1111111;
        let opcode = bits & opcode_mask;
        match opcode {
            OP_IMM | JALR | LOAD => Some(Instruction::parse_iformat(bits)),
            OP => Some(Instruction::parse_rformat(bits)),
            LUI | AUIPC => Some(Instruction::parse_uformat(bits)),
            JAL => Some(Instruction::parse_jformat(bits)),
            BRANCH => Some(Instruction::parse_bformat(bits)),
            STORE => Some(Instruction::parse_sformat(bits)),
            FENCE => todo!(),
            _ => None
        }
    }

    pub fn execute(self, register: &mut Register, memory: &mut Memory) {
        match self {
            IFormatInstruction { funct3, rd, rs1, imm, opcode } =>
                match opcode {
                    OP_IMM => {
                        match funct3 {
                            ADDI => { // Addi
                                let i = register.get(rs1);
                                register.put(rd, MixedIntegerOps::wrapping_add_signed(i, imm as i32));
                            },
                            SLLI => { // Slli
                                register.put(rd, register.get(rs1) << imm)
                            },
                            SLTI => { // Slti
                                let i = register.get(rs1) as i32;
                                if i < (imm as i32) {
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
                                        let shift = imm & 0b11111;
                                        let i = register.get(rs1);
                                        register.put(rd, i >> shift);
                                    },
                                    0b01 => {
                                        let shift = imm & 0b11111;
                                        let i = register.get(rs1) as i32;
                                        register.put(rd, (i >> shift) as u32);
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
                        }
                    },
                    JALR => {
                        let t = register.pc() + 4;
                        register.update_pc((register.get(rs1) as i32 + imm as i32) as usize);
                        if rd != 0 {
                            register.put(rd, t as u32);
                        }
                    },
                    LOAD => {
                        match funct3 {
                            LB => {
                                let m = register.get(rs1) as i32;
                                let offset = imm as i32;
                                let i = m + offset;
                                register.put(rd, memory[i as usize] as i8 as u32)
                            },
                            LH => {
                                let m = register.get(rs1) as i32;
                                let offset = imm as i32;
                                let i = m + offset;
                                register.put(rd, memory[i as usize] as i16 as u32)
                            },
                            LW => {
                                let m = register.get(rs1) as i32;
                                let offset = imm as i32;
                                let i = m + offset;
                                register.put(rd, memory[i as usize] as i32 as u32)
                            },
                            LBU => {
                                let m = register.get(rs1) as i32;
                                let offset = imm as i32;
                                let i = m + offset;
                                register.put(rd, memory[i as usize] as u8 as u32)
                            },
                            LHU => {
                                let m = register.get(rs1) as i32;
                                let offset = imm as i32;
                                let i = m + offset;
                                register.put(rd, memory[i as usize] as u16 as u32)
                            },
                            _ => return
                        }
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
                        let i = register.get(rs1) as i32;
                        let j = register.get(rs2) as i32;

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
                        let i = register.get(rs1) as i32;
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
                        register.put(rd, register.pc() as u32 + u_immediate);
                    },
                    _ => return
                },
            JFormatInstruction { imm, rd, opcode } =>
                match opcode {
                    JAL => {
                        if rd > 0 {
                            register.put(rd, register.pc() as u32 + 4);
                        }
                        register.update_pc(MixedIntegerOps::wrapping_add_signed(register.pc(), imm));
                    },
                    _ => return
                },
            BFormatInstruction { imm, rs1, rs2, funct3 } =>
                match funct3 {
                    BEQ => {
                        if register.get(rs1) == register.get(rs2) {
                            register.update_pc(MixedIntegerOps::wrapping_add_signed(register.pc(), imm));
                        }
                    },
                    BNE => {
                        if register.get(rs1) != register.get(rs2) {
                            register.update_pc(MixedIntegerOps::wrapping_add_signed(register.pc(), imm));
                        }
                    },
                    BLT => {
                        if (register.get(rs1) as i32) < (register.get(rs2) as i32) {
                            register.update_pc(MixedIntegerOps::wrapping_add_signed(register.pc(), imm));
                        }
                    },
                    BGE => {
                        if (register.get(rs1) as i32) >= (register.get(rs2) as i32) {
                            register.update_pc(MixedIntegerOps::wrapping_add_signed(register.pc(), imm));
                        }
                    },
                    BLTU => {
                        if register.get(rs1) < register.get(rs2) {
                            register.update_pc(MixedIntegerOps::wrapping_add_signed(register.pc(), imm));
                        }
                    },
                    BGEU => {
                        if register.get(rs1) >= register.get(rs2) {
                            register.update_pc(MixedIntegerOps::wrapping_add_signed(register.pc(), imm));
                        }
                    },
                    _ => return
                },
            SFormatInstruction { imm, rs1, rs2, funct3 } => {
                match funct3 {
                    SB => {
                        let m = (register.get(rs1) as i32 + imm) as usize;
                        memory[m] = register.get(rs2) as u8 as u32;
                    },
                    SH => {
                        let m = (register.get(rs1) as i32 + imm) as usize;
                        memory[m] = register.get(rs2) as u16 as u32;
                    },
                    SW => {
                        let m = (register.get(rs1) as i32 + imm) as usize;
                        memory[m] = register.get(rs2) as u32;
                    },
                    _ => return
                }
            }
        }
    }

    fn parse_iformat(bits: u32) -> Instruction {
        let opcode = bits & 0b1111111;
        let rd = (bits >> 7 & 0b11111) as usize;
        let funct3 = bits >> 12 & 0b111;
        let rs1 = (bits >> 15 & 0b11111) as usize;
        let imm: u32 = IImmediate::from_instruction(bits).into();
        let imm = imm as i16;

        IFormatInstruction {
            imm,
            rs1,
            funct3,
            rd,
            opcode
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
        let imm: u32 = UImmediate::from_instruction(bits).into();
        let imm = imm as i32;
        UFormatInstruction {
            imm,
            rd,
            opcode
        }
    }

    fn parse_jformat(bits: u32) -> Instruction {
        let opcode = bits & 0b1111111;
        let rd = (bits >> 7 & 0b11111) as usize;
        let imm: u32 = JImmediate::from_instruction(bits).into();
        let imm = imm as i32;

        JFormatInstruction {
            imm,
            rd,
            opcode
        }
    }

    fn parse_bformat(bits: u32) -> Instruction {
        let rs1 = (bits >> 15 & 0b11111) as usize;
        let rs2 = (bits >> 20 & 0b11111) as usize;
        let funct3 = (bits >> 12 & 0b111) as u32;
        let imm: u32 = BImmediate::from_instruction(bits).into();
        let imm = imm as i32;

        BFormatInstruction {
            imm,
            rs1,
            rs2,
            funct3
        }
    }

    fn parse_sformat(bits: u32) -> Instruction {
        let rs1 = (bits >> 15 & 0b11111) as usize;
        let rs2 = (bits >> 20 & 0b11111) as usize;
        let funct3 = (bits >> 12 & 0b111) as u32;

        let imm: u32 = SImmediate::from_instruction(bits).into();
        let imm = imm as i32;

        SFormatInstruction {
            imm,
            rs1,
            rs2,
            funct3
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::*;
    use crate::register::Register;

    #[test]
    fn test_add() {
        let mut register = Register::new();
        register.put(4, 0x7fffffff);
        register.put(24, 0x1);

        let mut memory = [0u32; 1024];

        let instruction = RFormatInstruction {
            rd: 25,
            funct3: 0b000,
            rs1: 4,
            rs2: 24,
            funct7: 0b0000000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(25), 0x80000000);
    }

    #[test]
    fn test_addi() {
        let mut register = Register::new();
        register.put(20, 0x20000000);

        let mut memory = [0u32; 1024];

        let instruction = IFormatInstruction {
            imm: -0x800,
            rs1: 20,
            funct3: ADDI,
            rd: 7,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(7), 0x1ffff800);
    }

    #[test]
    fn test_and() {
        let mut register = Register::new();
        register.put(10, 0x3);
        register.put(11, 0x55555556);

        let mut memory = [0u32; 1024];

        let instruction = RFormatInstruction {
            rd: 12,
            funct3: 0b111,
            rs1: 10,
            rs2: 11,
            funct7: 0b0000000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(12), 0x2);
    }

    #[test]
    fn test_andi() {
        let mut register = Register::new();
        register.put(10, 0x55555555);

        let mut memory = [0u32; 1024];

        let instruction = IFormatInstruction {
            imm: 0x334,
            rs1: 10,
            funct3: ANDI,
            rd: 11,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(11), 0x114);
    }

    #[test]
    fn test_auipc() {
        let mut register = Register::new();

        let mut memory = [0u32; 1024];

        let instruction = UFormatInstruction {
            imm: 0x100,
            rd: 10,
            opcode: AUIPC
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(10), 0x100000);
    }

    #[test]
    fn test_lui() {
        let mut register = Register::new();

        let mut memory = [0u32; 1024];

        let instruction = UFormatInstruction {
            imm: 0x3,
            rd: 13,
            opcode: LUI
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(13), 0x3000);
    }

    #[test]
    fn test_or() {
        let mut register = Register::new();
        register.put(8, 0x100000);
        register.put(26, 0x10);

        let mut memory = [0u32; 1024];

        let instruction = RFormatInstruction {
            rd: 26,
            funct3: 0b110,
            rs1: 8,
            rs2: 26,
            funct7: 0b0000000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(26), 0x100010);
    }

    #[test]
    fn test_ori() {
        let mut register = Register::new();
        register.put(17, 0x33333334);

        let mut memory = [0u32; 1024];

        let instruction = IFormatInstruction {
            imm: 0x7ff,
            rs1: 17,
            funct3: ORI,
            rd: 8,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(8), 0x333337ff);
    }

    #[test]
    fn test_sll() {
        let mut register = Register::new();
        register.put(12, 0x7fffffff);
        register.put(26, 0x15);

        let mut memory = [0u32; 1024];

        let instruction = RFormatInstruction {
            rd: 16,
            funct3: 0b001,
            rs1: 12,
            rs2: 26,
            funct7: 0b0000000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(16), 0xffe00000);
    }

    #[test]
    fn test_slli() {
        let mut register = Register::new();
        register.put(26, 0x66666666);

        let mut memory = [0u32; 1024];

        let instruction = IFormatInstruction {
            imm: 0xf,
            rs1: 26,
            funct3: SLLI,
            rd: 26,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(26), 0x33330000);
    }

    #[test]
    fn test_slt_equal() {
        let mut register = Register::new();
        register.put(26, 0x66666667);
        register.put(18, 0x66666667);

        let mut memory = [0u32; 1024];

        let instruction = RFormatInstruction {
            rd: 26,
            funct3: 0b010,
            rs1: 26,
            rs2: 18,
            funct7: 0b0000000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(26), 0x0);
    }

    #[test]
    fn test_slt_greater_than() {
        let mut register = Register::new();
        register.put(26, 0x66666667);
        register.put(18, 0x66666667);

        let mut memory = [0u32; 1024];

        let instruction = RFormatInstruction {
            rd: 26,
            funct3: 0b010,
            rs1: 26,
            rs2: 18,
            funct7: 0b0000000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(26), 0x0);
    }

    #[test]
    fn test_slt_less_than() {
        let mut register = Register::new();
        register.put(26, (-0x201i32) as u32);
        register.put(18, 0x5);

        let mut memory = [0u32; 1024];

        let instruction = RFormatInstruction {
            rd: 26,
            funct3: 0b010,
            rs1: 26,
            rs2: 18,
            funct7: 0b0000000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(26), 0x1);
    }

    #[test]
    fn test_slti_eq() {
        let mut register = Register::new();
        register.put(14, 0x10);

        let mut memory = [0u32; 1024];

        let instruction = IFormatInstruction {
            imm: 0x10,
            rs1: 14,
            funct3: SLTI,
            rd: 27,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(27), 0x0);
    }

    #[test]
    fn test_slti_gt() {
        let mut register = Register::new();
        register.put(25, -0x81i32 as u32);

        let mut memory = [0u32; 1024];

        let instruction = IFormatInstruction {
            imm: -0x800,
            rs1: 25,
            funct3: SLTI,
            rd: 12,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(12), 0x0);
    }

    #[test]
    fn test_slti_lt() {
        let mut register = Register::new();
        register.put(5, -0x1001i32 as u32);

        let mut memory = [0u32; 1024];

        let instruction = IFormatInstruction {
            imm: 0x0,
            rs1: 5,
            funct3: SLTI,
            rd: 5,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(5), 0x1);
    }

    #[test]
    fn test_sltiu_gt() {
        let mut register = Register::new();
        register.put(23, 0x400);

        let mut memory = [0u32; 1024];

        let instruction = IFormatInstruction {
            imm: 0x0,
            rs1: 23,
            funct3: SLTIU,
            rd: 28,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(28), 0x0);
    }

    #[test]
    fn test_sltiu_lt() {
        let mut register = Register::new();
        register.put(2, 0x800);

        let mut memory = [0u32; 1024];

        let instruction = IFormatInstruction {
            imm: 0xfff,
            rs1: 2,
            funct3: SLTIU,
            rd: 2,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(2), 0x1);
    }

    #[test]
    fn test_sltu_lt() {
        let mut register = Register::new();
        register.put(14, 0xfffffffe);
        register.put(24, 0xffffffff);

        let mut memory = [0u32; 1024];

        let instruction = RFormatInstruction {
            rd: 14,
            funct3: 0b011,
            rs1: 14,
            rs2: 24,
            funct7: 0b0000000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(14), 0x1);
    }

    #[test]
    fn test_sltu_gt() {
        let mut register = Register::new();
        register.put(5, 0xffffffff);
        register.put(14, 0x0);

        let mut memory = [0u32; 1024];

        let instruction = RFormatInstruction {
            rd: 19,
            funct3: 0b011,
            rs1: 5,
            rs2: 14,
            funct7: 0b0000000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(19), 0x0);
    }

    #[test]
    fn test_sra() {
        let mut register = Register::new();
        register.put(16, -0x80000000i32 as u32);
        register.put(27, 0x8);

        let mut memory = [0u32; 1024];

        let instruction = RFormatInstruction {
            rd: 16,
            funct3: 0b101,
            rs1: 16,
            rs2: 27,
            funct7: 0b0100000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(16), -0x800000i32 as u32)
    }

    #[test]
    fn test_srai() {
        let mut register = Register::new();
        register.put(31, -0x9i32 as u32);

        let mut memory = [0u32; 1024];

        let instruction = IFormatInstruction {
            imm: 0x9 + 0b010000000000, // adding discriminator
            rs1: 31,
            funct3: SRLI,
            rd: 25,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(25), -0x1i32 as u32)
    }

    #[test]
    fn test_srl() {
        let mut register = Register::new();
        register.put(26, -0x400001i32 as u32);
        register.put(11, 0xf);

        let mut memory = [0u32; 1024];

        let instruction = RFormatInstruction {
            rd: 11,
            funct3: 0b101,
            rs1: 26,
            rs2: 11,
            funct7: 0b0000000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(11), 0x1ff7f)
    }

    #[test]
    fn test_srli() {
        let mut register = Register::new();
        register.put(30, -0xb504i32 as u32);

        let mut memory = [0u32; 1024];

        let instruction = IFormatInstruction {
            imm: 0x2,
            rs1: 30,
            funct3: SRLI,
            rd: 8,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(8), 0x3fffd2bf)
    }

    #[test]
    fn test_sub() {
        let mut register = Register::new();
        register.put(24, 0x55555554);
        register.put(26, 0x6);

        let mut memory = [0u32; 1024];

        let instruction = RFormatInstruction {
            rd: 26,
            funct3: 0b000,
            rs1: 24,
            rs2: 26,
            funct7: 0b0100000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(26), 0x5555554e)
    }

    #[test]
    fn test_xor() {
        let mut register = Register::new();
        register.put(27, 0x66666665);
        register.put(24, 0x3);

        let mut memory = [0u32; 1024];

        let instruction = RFormatInstruction {
            rd: 24,
            funct3: 0b100,
            rs1: 27,
            rs2: 24,
            funct7: 0b0000000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(24), 0x66666666)
    }

    #[test]
    fn test_xori() {
        let mut register = Register::new();
        register.put(24, 0x33333334);

        let mut memory = [0u32; 1024];

        let instruction = IFormatInstruction {
            imm: -0x800,
            rs1: 24,
            funct3: XORI,
            rd: 10,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(10), 0xcccccb34)
    }

    #[test]
    fn test_lb() {
        let mut register = Register::new();
        register.put(24, 0xFF);

        let mut memory = [0u32; 1024];
        memory[0xFF] = 0xcccccb34;

        let instruction = IFormatInstruction {
            imm: 0x0,
            rs1: 24,
            funct3: LB,
            rd: 10,
            opcode: LOAD
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(10), 0b00000000_00000000_00000000_00110100)
    }

    #[test]
    fn test_lh() {
        let mut register = Register::new();
        register.put(24, 0xFF);

        let mut memory = [0u32; 1024];
        memory[0xFF] = 0xcccccb34;

        let instruction = IFormatInstruction {
            imm: 0x0,
            rs1: 24,
            funct3: LH,
            rd: 10,
            opcode: LOAD
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(10), 0b11111111_11111111_11001011_00110100)
    }

    #[test]
    fn test_lw() {
        let mut register = Register::new();
        register.put(24, 0xFF);

        let mut memory = [0u32; 1024];
        memory[0xFF] = 0xcccccb34;

        let instruction = IFormatInstruction {
            imm: 0x0,
            rs1: 24,
            funct3: LW,
            rd: 10,
            opcode: LOAD
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(10), 0b11001100_11001100_11001011_00110100)
    }

    #[test]
    fn test_lbu() {
        let mut register = Register::new();
        register.put(24, 0xFF);

        let mut memory = [0u32; 1024];
        memory[0xFF] = 0xcccccb34;

        let instruction = IFormatInstruction {
            imm: 0x0,
            rs1: 24,
            funct3: LBU,
            rd: 10,
            opcode: LOAD
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(10), 0b00000000_00000000_00000000_00110100)
    }

    #[test]
    fn test_lhu() {
        let mut register = Register::new();
        register.put(24, 0xFF);

        let mut memory = [0u32; 1024];
        memory[0xFF] = 0xcccccb34;

        let instruction = IFormatInstruction {
            imm: 0x0,
            rs1: 24,
            funct3: LHU,
            rd: 10,
            opcode: LOAD
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get(10), 0b00000000_00000000_11001011_00110100)
    }

    #[test]
    fn test_beq_true() {
        let mut register = Register::new();
        register.put(10, 0xFF);
        register.put(20, 0xFF);

        let mut memory = [0u32; 1024];

        let instruction = BFormatInstruction {
            imm: 100,
            rs1: 10,
            rs2: 20,
            funct3: BEQ
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.pc(), 100);
    }

    #[test]
    fn test_beq_false() {
        let mut register = Register::new();
        register.put(10, 0xFF);
        register.put(20, -100i32 as u32);

        let mut memory = [0u32; 1024];

        let instruction = BFormatInstruction {
            imm: 100,
            rs1: 10,
            rs2: 20,
            funct3: BEQ
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.pc(), 0);
    }

    #[test]
    fn test_bne_true() {
        let mut register = Register::new();
        register.put(10, 0xFF);
        register.put(20, -100i32 as u32);

        let mut memory = [0u32; 1024];

        let instruction = BFormatInstruction {
            imm: 100,
            rs1: 10,
            rs2: 20,
            funct3: BNE
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.pc(), 100);
    }

    #[test]
    fn test_bne_false() {
        let mut register = Register::new();
        register.put(10, 0xFF);
        register.put(20, 0xFF);

        let mut memory = [0u32; 1024];

        let instruction = BFormatInstruction {
            imm: 100,
            rs1: 10,
            rs2: 20,
            funct3: BNE
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.pc(), 0);
    }

    #[test]
    fn test_blt_true() {
        let mut register = Register::new();
        register.put(10, -100i32 as u32);
        register.put(20, 0xFF);

        let mut memory = [0u32; 1024];

        let instruction = BFormatInstruction {
            imm: 100,
            rs1: 10,
            rs2: 20,
            funct3: BLT
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.pc(), 100);
    }

    #[test]
    fn test_blt_false() {
        let mut register = Register::new();
        register.put(10, 0xFF);
        register.put(20, 0xFF);

        let mut memory = [0u32; 1024];

        let instruction = BFormatInstruction {
            imm: 100,
            rs1: 10,
            rs2: 20,
            funct3: BLT
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.pc(), 0);
    }

    #[test]
    fn test_bge_true() {
        let mut register = Register::new();
        register.put(10, 0xFF);
        register.put(20, -100i32 as u32);

        let mut memory = [0u32; 1024];

        let instruction = BFormatInstruction {
            imm: 100,
            rs1: 10,
            rs2: 20,
            funct3: BGE
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.pc(), 100);
    }

    #[test]
    fn test_bge_false() {
        let mut register = Register::new();
        register.put(10, 0xFF);
        register.put(20, 0xFFF);

        let mut memory = [0u32; 1024];

        let instruction = BFormatInstruction {
            imm: 100,
            rs1: 10,
            rs2: 20,
            funct3: BGE
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.pc(), 0);
    }

    #[test]
    fn test_bltu_true() {
        let mut register = Register::new();
        register.put(10, 0xFF);
        register.put(20, -100i32 as u32);

        let mut memory = [0u32; 1024];

        let instruction = BFormatInstruction {
            imm: 100,
            rs1: 10,
            rs2: 20,
            funct3: BLTU
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.pc(), 100);
    }

    #[test]
    fn test_bltu_false() {
        let mut register = Register::new();
        register.put(10, 0xFF);
        register.put(20, 0xFF);

        let mut memory = [0u32; 1024];

        let instruction = BFormatInstruction {
            imm: 100,
            rs1: 10,
            rs2: 20,
            funct3: BLTU
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.pc(), 0);
    }

    #[test]
    fn test_bgeu_true() {
        let mut register = Register::new();
        register.put(10, -100i32 as u32);
        register.put(20, 0xFF);

        let mut memory = [0u32; 1024];

        let instruction = BFormatInstruction {
            imm: 100,
            rs1: 10,
            rs2: 20,
            funct3: BGEU
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.pc(), 100);
    }

    #[test]
    fn test_bgeu_false() {
        let mut register = Register::new();
        register.put(10, 0xFF);
        register.put(20, 0xFFF);

        let mut memory = [0u32; 1024];

        let instruction = BFormatInstruction {
            imm: 100,
            rs1: 10,
            rs2: 20,
            funct3: BGEU
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.pc(), 0);
    }

    #[test]
    fn test_sb() {
        let mut register = Register::new();
        register.put(10, 0x100);
        register.put(20, 0xFFFFFF);

        let mut memory = [0u32; 1024];

        let instruction = SFormatInstruction {
            imm: 128,
            rs1: 10,
            rs2: 20,
            funct3: SB
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(memory[384], 0xFF);
    }

    #[test]
    fn test_sh() {
        let mut register = Register::new();
        register.put(10, 0x100);
        register.put(20, 0xFFFFFF);

        let mut memory = [0u32; 1024];

        let instruction = SFormatInstruction {
            imm: 128,
            rs1: 10,
            rs2: 20,
            funct3: SH
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(memory[384], 0xFFFF);
    }

    #[test]
    fn test_sw() {
        let mut register = Register::new();
        register.put(10, 0x100);
        register.put(20, 0xFFFFFF);

        let mut memory = [0u32; 1024];

        let instruction = SFormatInstruction {
            imm: 128,
            rs1: 10,
            rs2: 20,
            funct3: SW
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(memory[384], 0xFFFFFF);
    }
}
