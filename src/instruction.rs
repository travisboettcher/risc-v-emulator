use crate::immediates::{BImmediate, IImmediate, Immediate, JImmediate, SImmediate, UImmediate};
use crate::register::Register;
use crate::math_utils::MixedIntegerOps;
use crate::error::{EmulatorError, Result};

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

type Memory = [u8; 1024];

pub trait Instruction {
    fn execute(self, register: &mut Register, memory: &mut Memory) -> Result<()>;
    fn parse(bits: u32) -> InstructionEnum;
}

#[derive(Debug)]
pub struct IFormatInstruction {
    imm: i16,
    rs1: usize,
    funct3: u32,
    rd: usize,
    opcode: u32
}

impl Instruction for IFormatInstruction {
    fn execute(self, register: &mut Register, memory: &mut Memory) -> Result<()> {
        match self.opcode {
            OP_IMM => {
                match self.funct3 {
                    ADDI => { // Addi
                        let i = register.get_unchecked(self.rs1);
                        register.put_unchecked(self.rd, MixedIntegerOps::wrapping_add_signed(i, self.imm as i32));
                    },
                    SLLI => { // Slli
                        register.put_unchecked(self.rd, register.get_unchecked(self.rs1) << self.imm)
                    },
                    SLTI => { // Slti
                        let i = register.get_unchecked(self.rs1) as i32;
                        if i < (self.imm as i32) {
                            register.put_unchecked(self.rd, 1);
                        } else {
                            register.put_unchecked(self.rd, 0);
                        }
                    },
                    SLTIU => { // Sltiu
                        let i = register.get_unchecked(self.rs1);
                        if i < (self.imm as u32) {
                            register.put_unchecked(self.rd, 1);
                        } else {
                            register.put_unchecked(self.rd, 0);
                        }
                    },
                    XORI => { // Xori
                        let i = register.get_unchecked(self.rs1);
                        register.put_unchecked(self.rd, i ^ (self.imm as u32));
                    },
                    SRLI => { // Srli and Srai
                        // need to discriminate between srli and srai
                        let discriminator = self.imm >> 10;
                        match discriminator {
                            0b00 => {
                                let shift = self.imm & 0b11111;
                                let i = register.get_unchecked(self.rs1);
                                register.put_unchecked(self.rd, i >> shift);
                            },
                            0b01 => {
                                let shift = self.imm & 0b11111;
                                let i = register.get_unchecked(self.rs1) as i32;
                                register.put_unchecked(self.rd, (i >> shift) as u32);
                            },
                            _ => return Err(EmulatorError::InvalidInstruction(0))
                        }
                    }
                    ORI => { // Ori
                        let i = register.get_unchecked(self.rs1);
                        register.put_unchecked(self.rd, i | (self.imm as u32));
                    },
                    ANDI => { // Andi
                        let i = register.get_unchecked(self.rs1);
                        register.put_unchecked(self.rd, i & (self.imm as u32));
                    },
                    _ => return Ok(())
                }
            },
            JALR => {
                let t = register.pc();
                register.update_pc((register.get_unchecked(self.rs1) as i32 + self.imm as i32) as usize);
                if self.rd != 0 {
                    register.put_unchecked(self.rd, t as u32);
                }
            },
            LOAD => {
                match self.funct3 {
                    LB => {
                        let m = register.get_unchecked(self.rs1) as i32;
                        let offset = self.imm as i32;
                        let i = m + offset;
                        register.put_unchecked(self.rd, memory[i as usize] as i8 as u32)
                    },
                    LH => {
                        let m = register.get_unchecked(self.rs1) as i32;
                        let offset = self.imm as i32;
                        let i = m + offset;

                        let mut bits: [u8; 4] = [0u8; 4];
                        bits[0] = 0xFF;
                        bits[1] = 0xFF;
                        bits[2] = memory[i as usize];
                        bits[3] = memory[(i + 1)as usize];
                        register.put_unchecked(self.rd, u32::from_be_bytes(bits))
                    },
                    LW => {
                        let m = register.get_unchecked(self.rs1) as i32;
                        let offset = self.imm as i32;
                        let i = m + offset;

                        let mut bits: [u8; 4] = [0u8; 4];
                        bits.clone_from_slice(&memory[i as usize..(i + 4) as usize]);
                        register.put_unchecked(self.rd, u32::from_be_bytes(bits))
                    },
                    LBU => {
                        let m = register.get_unchecked(self.rs1) as i32;
                        let offset = self.imm as i32;
                        let i = m + offset;
                        register.put_unchecked(self.rd, memory[i as usize] as u8 as u32)
                    },
                    LHU => {
                        let m = register.get_unchecked(self.rs1) as i32;
                        let offset = self.imm as i32;
                        let i = m + offset;

                        let mut bits: [u8; 4] = [0u8; 4];
                        bits[2] = memory[i as usize];
                        bits[3] = memory[(i + 1)as usize];
                        register.put_unchecked(self.rd, u32::from_be_bytes(bits))
                    },
                    _ => return Ok(())
                }
            },
            _ => return Ok(())
        }
        Ok(())
    }

    fn parse(bits: u32) -> InstructionEnum {
        let opcode = bits & 0b1111111;
        let rd = (bits >> 7 & 0b11111) as usize;
        let funct3 = bits >> 12 & 0b111;
        let rs1 = (bits >> 15 & 0b11111) as usize;
        let imm: u32 = IImmediate::from_instruction(bits).into();
        let imm = imm as i16;

        InstructionEnum::IFormatInstruction {
            instruction: IFormatInstruction {
                imm,
                rs1,
                funct3,
                rd,
                opcode
            }
        }
    }
}

#[derive(Debug)]
pub struct JFormatInstruction {
    imm: i32,
    rd: usize,
    opcode: u32
}

impl Instruction for JFormatInstruction {
    fn execute(self, register: &mut Register, memory: &mut Memory) -> Result<()> {
        match self.opcode {
            JAL => {
                if self.rd > 0 {
                    register.put_unchecked(self.rd, register.pc() as u32);
                }
                register.update_pc(MixedIntegerOps::wrapping_add_signed(register.pc(), self.imm));
            },
            _ => return Ok(())
        }
        Ok(())
    }

    fn parse(bits: u32) -> InstructionEnum {
        let opcode = bits & 0b1111111;
        let rd = (bits >> 7 & 0b11111) as usize;
        let imm: u32 = JImmediate::from_instruction(bits).into();
        let imm = imm as i32;

        InstructionEnum::JFormatInstruction {
            instruction: JFormatInstruction {
                imm,
                rd,
                opcode
            }
        }
    }
}

#[derive(Debug)]
pub struct RFormatInstruction {
    rd: usize,
    funct3: u32,
    rs1: usize,
    rs2: usize,
    funct7: u32
}

impl Instruction for RFormatInstruction {
    fn execute(self, register: &mut Register, memory: &mut Memory) -> Result<()> {
        let funct = (self.funct7 << 3) + self.funct3;
        match funct {
            ADD => { // Add
                let i = register.get_unchecked(self.rs1);
                let j = register.get_unchecked(self.rs2);
                register.put_unchecked(self.rd, i + j);
            },
            SUB => {
                let i = register.get_unchecked(self.rs1);
                let j = register.get_unchecked(self.rs2);
                register.put_unchecked(self.rd, i - j);
            },
            SLL => {
                let i = register.get_unchecked(self.rs1);
                let j = register.get_unchecked(self.rs2) & 0b11111;
                register.put_unchecked(self.rd, i << j)
            },
            SLT => {
                let i = register.get_unchecked(self.rs1) as i32;
                let j = register.get_unchecked(self.rs2) as i32;

                if i < j {
                    register.put_unchecked(self.rd, 1);
                } else {
                    register.put_unchecked(self.rd, 0);
                }
            },
            SLTU => {
                let i = register.get_unchecked(self.rs1);
                let j = register.get_unchecked(self.rs2);
                if i < j {
                    register.put_unchecked(self.rd, 1);
                } else {
                    register.put_unchecked(self.rd, 0);
                }
            },
            XOR => {
                let i = register.get_unchecked(self.rs1);
                let j = register.get_unchecked(self.rs2);
                register.put_unchecked(self.rd, i ^ j);
            },
            SRL => {
                let i = register.get_unchecked(self.rs1);
                let j = register.get_unchecked(self.rs2) & 0b11111;
                register.put_unchecked(self.rd, i >> j);
            },
            SRA => {
                let i = register.get_unchecked(self.rs1) as i32;
                let j = register.get_unchecked(self.rs2) & 0b11111;
                register.put_unchecked(self.rd, (i >> j) as u32);
            },
            OR => {
                let i = register.get_unchecked(self.rs1);
                let j = register.get_unchecked(self.rs2);
                register.put_unchecked(self.rd, i | j);
            },
            AND => {
                let i = register.get_unchecked(self.rs1);
                let j = register.get_unchecked(self.rs2);
                println!("i: {}, j: {}, i & j: {}", i, j, i&j);
                register.put_unchecked(self.rd, i & j);
            }
            _ => return Ok(())
        }
        Ok(())
    }

    fn parse(bits: u32) -> InstructionEnum {
        let rd = (bits >> 7 & 0b11111) as usize;
        let funct3 = bits >> 12 & 0b111;
        let rs1 = (bits >> 15 & 0b11111) as usize;
        let rs2 = (bits >> 20 & 0b11111) as usize;
        let funct7 = bits >> 25;

        InstructionEnum::RFormatInstruction {
            instruction: RFormatInstruction {
                rs1,
                rs2,
                funct3,
                funct7,
                rd
            }
        }
    }
}

#[derive(Debug)]
pub struct UFormatInstruction {
    imm: i32,
    rd: usize,
    opcode: u32
}

impl Instruction for UFormatInstruction {
    fn execute(self, register: &mut Register, memory: &mut Memory) -> Result<()> {
        match self.opcode {
            LUI => {
                register.put_unchecked(self.rd, (self.imm as u32) << 12);
            },
            AUIPC => {
                let u_immediate = (self.imm as u32) << 12;
                register.put_unchecked(self.rd, register.pc() as u32 + u_immediate);
            },
            _ => return Ok(())
        }
        Ok(())
    }

    fn parse(bits: u32) -> InstructionEnum {
        let opcode = bits & 0b1111111;
        let rd = (bits >> 7 & 0b11111) as usize;
        let imm: u32 = UImmediate::from_instruction(bits).into();
        let imm = imm as i32;

        InstructionEnum::UFormatInstruction {
            instruction: UFormatInstruction {
                imm,
                rd,
                opcode
            }
        }
    }
}

#[derive(Debug)]
pub struct BFormatInstruction {
    imm: i32,
    rs1: usize,
    rs2: usize,
    funct3: u32
}

impl Instruction for BFormatInstruction {
    fn execute(self, register: &mut Register, memory: &mut Memory) -> Result<()> {
        match self.funct3 {
            BEQ => {
                if register.get_unchecked(self.rs1) == register.get_unchecked(self.rs2) {
                    register.update_pc(MixedIntegerOps::wrapping_add_signed(register.pc(), self.imm));
                }
            },
            BNE => {
                if register.get_unchecked(self.rs1) != register.get_unchecked(self.rs2) {
                    register.update_pc(MixedIntegerOps::wrapping_add_signed(register.pc(), self.imm));
                }
            },
            BLT => {
                if (register.get_unchecked(self.rs1) as i32) < (register.get_unchecked(self.rs2) as i32) {
                    register.update_pc(MixedIntegerOps::wrapping_add_signed(register.pc(), self.imm));
                }
            },
            BGE => {
                if (register.get_unchecked(self.rs1) as i32) >= (register.get_unchecked(self.rs2) as i32) {
                    register.update_pc(MixedIntegerOps::wrapping_add_signed(register.pc(), self.imm));
                }
            },
            BLTU => {
                if register.get_unchecked(self.rs1) < register.get_unchecked(self.rs2) {
                    register.update_pc(MixedIntegerOps::wrapping_add_signed(register.pc(), self.imm));
                }
            },
            BGEU => {
                if register.get_unchecked(self.rs1) >= register.get_unchecked(self.rs2) {
                    register.update_pc(MixedIntegerOps::wrapping_add_signed(register.pc(), self.imm));
                }
            },
            _ => return Ok(())
        }
        Ok(())
    }

    fn parse(bits: u32) -> InstructionEnum {
        let rs1 = (bits >> 15 & 0b11111) as usize;
        let rs2 = (bits >> 20 & 0b11111) as usize;
        let funct3 = (bits >> 12 & 0b111) as u32;
        let imm: u32 = BImmediate::from_instruction(bits).into();
        let imm = imm as i32;

        InstructionEnum::BFormatInstruction {
            instruction: BFormatInstruction {
                imm,
                rs1,
                rs2,
                funct3
            }
        }
    }
}

#[derive(Debug)]
pub struct SFormatInstruction {
    imm: i32,
    rs1: usize,
    rs2: usize,
    funct3: u32
}

impl Instruction for SFormatInstruction {
    fn execute(self, register: &mut Register, memory: &mut Memory) -> Result<()> {
        match self.funct3 {
            SB => {
                let m = (register.get_unchecked(self.rs1) as i32 + self.imm) as usize;
                memory[m] = register.get_unchecked(self.rs2).to_be_bytes()[3];
            },
            SH => {
                let m = (register.get_unchecked(self.rs1) as i32 + self.imm) as usize;
                let r = register.get_unchecked(self.rs2).to_be_bytes();
                memory[m] = r[2];
                memory[m + 1] = r[3]
            },
            SW => {
                let m = (register.get_unchecked(self.rs1) as i32 + self.imm) as usize;
                let r = register.get_unchecked(self.rs2).to_be_bytes();
                memory[m] = r[0];
                memory[m + 1] = r[1];
                memory[m + 2] = r[2];
                memory[m + 3] = r[3];
            },
            _ => return Ok(())
        }
        Ok(())
    }

    fn parse(bits: u32) -> InstructionEnum {
        let rs1 = (bits >> 15 & 0b11111) as usize;
        let rs2 = (bits >> 20 & 0b11111) as usize;
        let funct3 = (bits >> 12 & 0b111) as u32;

        let imm: u32 = SImmediate::from_instruction(bits).into();
        let imm = imm as i32;

        InstructionEnum::SFormatInstruction {
            instruction: SFormatInstruction {
            imm,
            rs1,
            rs2,
            funct3
        }
        }
    }
}

#[derive(Debug)]
pub enum InstructionEnum {
    IFormatInstruction {
        instruction: IFormatInstruction
    },
    JFormatInstruction {
        instruction: JFormatInstruction
    },
    RFormatInstruction {
        instruction: RFormatInstruction
    },
    UFormatInstruction {
        instruction: UFormatInstruction
    },
    BFormatInstruction {
        instruction: BFormatInstruction
    },
    SFormatInstruction {
        instruction: SFormatInstruction
    }
}

// Implement SCALL/SBREAK/CSRR* with a single SYSTEM instruction that always traps
// Implement FENCE and FENCE.I as NOPs

pub fn from(bits: [u8; 4]) -> Result<InstructionEnum> {
    let bits = u32::from_be_bytes(bits);
    let opcode_mask = 0b1111111;
    let opcode = bits & opcode_mask;
    match opcode {
        OP_IMM | JALR | LOAD => Ok(IFormatInstruction::parse(bits)),
        OP => Ok(RFormatInstruction::parse(bits)),
        LUI | AUIPC => Ok(UFormatInstruction::parse(bits)),
        JAL => Ok(JFormatInstruction::parse(bits)),
        BRANCH => Ok(BFormatInstruction::parse(bits)),
        STORE => Ok(SFormatInstruction::parse(bits)),
        FENCE => Err(EmulatorError::InvalidInstruction(bits)),
        _ => Err(EmulatorError::InvalidInstruction(bits))
    }
}

impl InstructionEnum {
    pub fn execute(self, register: &mut Register, memory: &mut Memory) -> Result<()> {
        match self {
            InstructionEnum::IFormatInstruction { instruction } => {
                instruction.execute(register, memory)
            }
            InstructionEnum::JFormatInstruction { instruction } => {
                instruction.execute(register, memory)
            }
            InstructionEnum::RFormatInstruction { instruction } => {
                instruction.execute(register, memory)
            }
            InstructionEnum::UFormatInstruction { instruction } => {
                instruction.execute(register, memory)
            }
            InstructionEnum::BFormatInstruction { instruction } => {
                instruction.execute(register, memory)
            }
            InstructionEnum::SFormatInstruction { instruction } => {
                instruction.execute(register, memory)
            }
        }
    }

    pub fn should_end(&self, register: &Register) -> bool {
        match self {
            InstructionEnum::IFormatInstruction { instruction: IFormatInstruction { opcode, rd, rs1, .. }} => {
                *opcode == JALR && *rd == 0 && *rs1 == 1 && register.get_unchecked(*rs1) == 0
            }
            _ => false
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
        register.put_unchecked(4, 0x7fffffff);
        register.put_unchecked(24, 0x1);

        let mut memory = [0u8; 1024];

        let instruction = RFormatInstruction {
            rd: 25,
            funct3: 0b000,
            rs1: 4,
            rs2: 24,
            funct7: 0b0000000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(25), 0x80000000);
    }

    #[test]
    fn test_addi() {
        let mut register = Register::new();
        register.put_unchecked(20, 0x20000000);

        let mut memory = [0u8; 1024];

        let instruction = IFormatInstruction {
            imm: -0x800,
            rs1: 20,
            funct3: ADDI,
            rd: 7,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(7), 0x1ffff800);
    }

    #[test]
    fn test_and() {
        let mut register = Register::new();
        register.put_unchecked(10, 0x3);
        register.put_unchecked(11, 0x55555556);

        let mut memory = [0u8; 1024];

        let instruction = RFormatInstruction {
            rd: 12,
            funct3: 0b111,
            rs1: 10,
            rs2: 11,
            funct7: 0b0000000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(12), 0x2);
    }

    #[test]
    fn test_andi() {
        let mut register = Register::new();
        register.put_unchecked(10, 0x55555555);

        let mut memory = [0u8; 1024];

        let instruction = IFormatInstruction {
            imm: 0x334,
            rs1: 10,
            funct3: ANDI,
            rd: 11,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(11), 0x114);
    }

    #[test]
    fn test_auipc() {
        let mut register = Register::new();

        let mut memory = [0u8; 1024];

        let instruction = UFormatInstruction {
            imm: 0x100,
            rd: 10,
            opcode: AUIPC
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(10), 0x100000);
    }

    #[test]
    fn test_lui() {
        let mut register = Register::new();

        let mut memory = [0u8; 1024];

        let instruction = UFormatInstruction {
            imm: 0x3,
            rd: 13,
            opcode: LUI
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(13), 0x3000);
    }

    #[test]
    fn test_or() {
        let mut register = Register::new();
        register.put_unchecked(8, 0x100000);
        register.put_unchecked(26, 0x10);

        let mut memory = [0u8; 1024];

        let instruction = RFormatInstruction {
            rd: 26,
            funct3: 0b110,
            rs1: 8,
            rs2: 26,
            funct7: 0b0000000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(26), 0x100010);
    }

    #[test]
    fn test_ori() {
        let mut register = Register::new();
        register.put_unchecked(17, 0x33333334);

        let mut memory = [0u8; 1024];

        let instruction = IFormatInstruction {
            imm: 0x7ff,
            rs1: 17,
            funct3: ORI,
            rd: 8,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(8), 0x333337ff);
    }

    #[test]
    fn test_sll() {
        let mut register = Register::new();
        register.put_unchecked(12, 0x7fffffff);
        register.put_unchecked(26, 0x15);

        let mut memory = [0u8; 1024];

        let instruction = RFormatInstruction {
            rd: 16,
            funct3: 0b001,
            rs1: 12,
            rs2: 26,
            funct7: 0b0000000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(16), 0xffe00000);
    }

    #[test]
    fn test_slli() {
        let mut register = Register::new();
        register.put_unchecked(26, 0x66666666);

        let mut memory = [0u8; 1024];

        let instruction = IFormatInstruction {
            imm: 0xf,
            rs1: 26,
            funct3: SLLI,
            rd: 26,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(26), 0x33330000);
    }

    #[test]
    fn test_slt_equal() {
        let mut register = Register::new();
        register.put_unchecked(26, 0x66666667);
        register.put_unchecked(18, 0x66666667);

        let mut memory = [0u8; 1024];

        let instruction = RFormatInstruction {
            rd: 26,
            funct3: 0b010,
            rs1: 26,
            rs2: 18,
            funct7: 0b0000000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(26), 0x0);
    }

    #[test]
    fn test_slt_greater_than() {
        let mut register = Register::new();
        register.put_unchecked(26, 0x66666667);
        register.put_unchecked(18, 0x66666667);

        let mut memory = [0u8; 1024];

        let instruction = RFormatInstruction {
            rd: 26,
            funct3: 0b010,
            rs1: 26,
            rs2: 18,
            funct7: 0b0000000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(26), 0x0);
    }

    #[test]
    fn test_slt_less_than() {
        let mut register = Register::new();
        register.put_unchecked(26, (-0x201i32) as u32);
        register.put_unchecked(18, 0x5);

        let mut memory = [0u8; 1024];

        let instruction = RFormatInstruction {
            rd: 26,
            funct3: 0b010,
            rs1: 26,
            rs2: 18,
            funct7: 0b0000000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(26), 0x1);
    }

    #[test]
    fn test_slti_eq() {
        let mut register = Register::new();
        register.put_unchecked(14, 0x10);

        let mut memory = [0u8; 1024];

        let instruction = IFormatInstruction {
            imm: 0x10,
            rs1: 14,
            funct3: SLTI,
            rd: 27,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(27), 0x0);
    }

    #[test]
    fn test_slti_gt() {
        let mut register = Register::new();
        register.put_unchecked(25, -0x81i32 as u32);

        let mut memory = [0u8; 1024];

        let instruction = IFormatInstruction {
            imm: -0x800,
            rs1: 25,
            funct3: SLTI,
            rd: 12,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(12), 0x0);
    }

    #[test]
    fn test_slti_lt() {
        let mut register = Register::new();
        register.put_unchecked(5, -0x1001i32 as u32);

        let mut memory = [0u8; 1024];

        let instruction = IFormatInstruction {
            imm: 0x0,
            rs1: 5,
            funct3: SLTI,
            rd: 5,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(5), 0x1);
    }

    #[test]
    fn test_sltiu_gt() {
        let mut register = Register::new();
        register.put_unchecked(23, 0x400);

        let mut memory = [0u8; 1024];

        let instruction = IFormatInstruction {
            imm: 0x0,
            rs1: 23,
            funct3: SLTIU,
            rd: 28,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(28), 0x0);
    }

    #[test]
    fn test_sltiu_lt() {
        let mut register = Register::new();
        register.put_unchecked(2, 0x800);

        let mut memory = [0u8; 1024];

        let instruction = IFormatInstruction {
            imm: 0xfff,
            rs1: 2,
            funct3: SLTIU,
            rd: 2,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(2), 0x1);
    }

    #[test]
    fn test_sltu_lt() {
        let mut register = Register::new();
        register.put_unchecked(14, 0xfffffffe);
        register.put_unchecked(24, 0xffffffff);

        let mut memory = [0u8; 1024];

        let instruction = RFormatInstruction {
            rd: 14,
            funct3: 0b011,
            rs1: 14,
            rs2: 24,
            funct7: 0b0000000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(14), 0x1);
    }

    #[test]
    fn test_sltu_gt() {
        let mut register = Register::new();
        register.put_unchecked(5, 0xffffffff);
        register.put_unchecked(14, 0x0);

        let mut memory = [0u8; 1024];

        let instruction = RFormatInstruction {
            rd: 19,
            funct3: 0b011,
            rs1: 5,
            rs2: 14,
            funct7: 0b0000000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(19), 0x0);
    }

    #[test]
    fn test_sra() {
        let mut register = Register::new();
        register.put_unchecked(16, -0x80000000i32 as u32);
        register.put_unchecked(27, 0x8);

        let mut memory = [0u8; 1024];

        let instruction = RFormatInstruction {
            rd: 16,
            funct3: 0b101,
            rs1: 16,
            rs2: 27,
            funct7: 0b0100000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(16), -0x800000i32 as u32)
    }

    #[test]
    fn test_srai() {
        let mut register = Register::new();
        register.put_unchecked(31, -0x9i32 as u32);

        let mut memory = [0u8; 1024];

        let instruction = IFormatInstruction {
            imm: 0x9 + 0b010000000000, // adding discriminator
            rs1: 31,
            funct3: SRLI,
            rd: 25,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(25), -0x1i32 as u32)
    }

    #[test]
    fn test_srl() {
        let mut register = Register::new();
        register.put_unchecked(26, -0x400001i32 as u32);
        register.put_unchecked(11, 0xf);

        let mut memory = [0u8; 1024];

        let instruction = RFormatInstruction {
            rd: 11,
            funct3: 0b101,
            rs1: 26,
            rs2: 11,
            funct7: 0b0000000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(11), 0x1ff7f)
    }

    #[test]
    fn test_srli() {
        let mut register = Register::new();
        register.put_unchecked(30, -0xb504i32 as u32);

        let mut memory = [0u8; 1024];

        let instruction = IFormatInstruction {
            imm: 0x2,
            rs1: 30,
            funct3: SRLI,
            rd: 8,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(8), 0x3fffd2bf)
    }

    #[test]
    fn test_sub() {
        let mut register = Register::new();
        register.put_unchecked(24, 0x55555554);
        register.put_unchecked(26, 0x6);

        let mut memory = [0u8; 1024];

        let instruction = RFormatInstruction {
            rd: 26,
            funct3: 0b000,
            rs1: 24,
            rs2: 26,
            funct7: 0b0100000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(26), 0x5555554e)
    }

    #[test]
    fn test_xor() {
        let mut register = Register::new();
        register.put_unchecked(27, 0x66666665);
        register.put_unchecked(24, 0x3);

        let mut memory = [0u8; 1024];

        let instruction = RFormatInstruction {
            rd: 24,
            funct3: 0b100,
            rs1: 27,
            rs2: 24,
            funct7: 0b0000000
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(24), 0x66666666)
    }

    #[test]
    fn test_xori() {
        let mut register = Register::new();
        register.put_unchecked(24, 0x33333334);

        let mut memory = [0u8; 1024];

        let instruction = IFormatInstruction {
            imm: -0x800,
            rs1: 24,
            funct3: XORI,
            rd: 10,
            opcode: OP_IMM
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(10), 0xcccccb34)
    }

    #[test]
    fn test_lb() {
        let mut register = Register::new();
        register.put_unchecked(24, 0xFF);

        let mut memory = [0u8; 1024];
        memory[0xFF] = 0x34;
        memory[0x100] = 0xcb;
        memory[0x101] = 0xcc;
        memory[0x102] = 0xcc;

        let instruction = IFormatInstruction {
            imm: 0x0,
            rs1: 24,
            funct3: LB,
            rd: 10,
            opcode: LOAD
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(10), 0b00000000_00000000_00000000_00110100)
    }

    #[test]
    fn test_lh() {
        let mut register = Register::new();
        register.put_unchecked(24, 0x101);

        let mut memory = [0u8; 1024];
        memory[0xFF] = 0xcc;
        memory[0x100] = 0xcc;
        memory[0x101] = 0xcb;
        memory[0x102] = 0x34;

        let instruction = IFormatInstruction {
            imm: 0x0,
            rs1: 24,
            funct3: LH,
            rd: 10,
            opcode: LOAD
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(10), 0b11111111_11111111_11001011_00110100)
    }

    #[test]
    fn test_lw() {
        let mut register = Register::new();
        register.put_unchecked(24, 0xFF);

        let mut memory = [0u8; 1024];
        memory[0xFF] = 0xcc;
        memory[0x100] = 0xcc;
        memory[0x101] = 0xcb;
        memory[0x102] = 0x34;

        let instruction = IFormatInstruction {
            imm: 0x0,
            rs1: 24,
            funct3: LW,
            rd: 10,
            opcode: LOAD
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(10), 0b11001100_11001100_11001011_00110100)
    }

    #[test]
    fn test_lbu() {
        let mut register = Register::new();
        register.put_unchecked(24, 0xFF);

        let mut memory = [0u8; 1024];
        memory[0xFF] = 0x34;
        memory[0x100] = 0xcb;
        memory[0x101] = 0xcc;
        memory[0x102] = 0xcc;

        let instruction = IFormatInstruction {
            imm: 0x0,
            rs1: 24,
            funct3: LBU,
            rd: 10,
            opcode: LOAD
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(10), 0b00000000_00000000_00000000_00110100)
    }

    #[test]
    fn test_lhu() {
        let mut register = Register::new();
        register.put_unchecked(24, 0x101);

        let mut memory = [0u8; 1024];
        memory[0xFF] = 0xcc;
        memory[0x100] = 0xcc;
        memory[0x101] = 0xcb;
        memory[0x102] = 0x34;

        let instruction = IFormatInstruction {
            imm: 0x0,
            rs1: 24,
            funct3: LHU,
            rd: 10,
            opcode: LOAD
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(register.get_unchecked(10), 0b00000000_00000000_11001011_00110100)
    }

    #[test]
    fn test_beq_true() {
        let mut register = Register::new();
        register.put_unchecked(10, 0xFF);
        register.put_unchecked(20, 0xFF);

        let mut memory = [0u8; 1024];

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
        register.put_unchecked(10, 0xFF);
        register.put_unchecked(20, -100i32 as u32);

        let mut memory = [0u8; 1024];

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
        register.put_unchecked(10, 0xFF);
        register.put_unchecked(20, -100i32 as u32);

        let mut memory = [0u8; 1024];

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
        register.put_unchecked(10, 0xFF);
        register.put_unchecked(20, 0xFF);

        let mut memory = [0u8; 1024];

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
        register.put_unchecked(10, -100i32 as u32);
        register.put_unchecked(20, 0xFF);

        let mut memory = [0u8; 1024];

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
        register.put_unchecked(10, 0xFF);
        register.put_unchecked(20, 0xFF);

        let mut memory = [0u8; 1024];

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
        register.put_unchecked(10, 0xFF);
        register.put_unchecked(20, -100i32 as u32);

        let mut memory = [0u8; 1024];

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
        register.put_unchecked(10, 0xFF);
        register.put_unchecked(20, 0xFFF);

        let mut memory = [0u8; 1024];

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
        register.put_unchecked(10, 0xFF);
        register.put_unchecked(20, -100i32 as u32);

        let mut memory = [0u8; 1024];

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
        register.put_unchecked(10, 0xFF);
        register.put_unchecked(20, 0xFF);

        let mut memory = [0u8; 1024];

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
        register.put_unchecked(10, -100i32 as u32);
        register.put_unchecked(20, 0xFF);

        let mut memory = [0u8; 1024];

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
        register.put_unchecked(10, 0xFF);
        register.put_unchecked(20, 0xFFF);

        let mut memory = [0u8; 1024];

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
        register.put_unchecked(10, 0x100);
        register.put_unchecked(20, 0xFFFFFF);

        let mut memory = [0u8; 1024];

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
        register.put_unchecked(10, 0x100);
        register.put_unchecked(20, 0xFFFFFF);

        let mut memory = [0u8; 1024];

        let instruction = SFormatInstruction {
            imm: 128,
            rs1: 10,
            rs2: 20,
            funct3: SH
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(memory[384], 0xFF);
        assert_eq!(memory[385], 0xFF)
    }

    #[test]
    fn test_sw() {
        let mut register = Register::new();
        register.put_unchecked(10, 0x100);
        register.put_unchecked(20, 0xFFFFFF);

        let mut memory = [0u8; 1024];

        let instruction = SFormatInstruction {
            imm: 128,
            rs1: 10,
            rs2: 20,
            funct3: SW
        };
        instruction.execute(&mut register, &mut memory);

        assert_eq!(memory[384], 0x00);
        assert_eq!(memory[385], 0xFF);
        assert_eq!(memory[386], 0xFF);
        assert_eq!(memory[387], 0xFF);
    }
}
