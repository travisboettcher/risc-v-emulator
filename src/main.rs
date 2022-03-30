use std::u16;
use crate::Instructions::*;

#[derive(Debug)]
struct Register {
    x: [usize; 32],
    pc: usize
}

impl Register {
    pub fn put(&mut self, index: usize, value: usize) {
        if index == 0 {
            panic!("Cannot modify the register at index 0");
        }

        if index > 31 {
            panic!("The register only has a length of 32, tried to modify index {}", index)
        }

        self.x[index] = value;
    }

    pub fn get(&self, index: usize) -> usize {
        if index > 31 {
            panic!("The register only has a length of 32, tried to access index {}", index)
        }

        self.x[index]
    }
}

#[derive(Debug)]
enum Instructions {
    AddI {
        rs1: usize,
        rd: usize,
        imm: i16 // this should be a 12-bit signed integer
    },
    Add {
        rs1: usize,
        rs2: usize,
        rd: usize
    },
    Slti {
        rs1: usize,
        rd: usize,
        imm: i16
    },
    Sltiu {
        rs1: usize,
        rd: usize,
        imm: i16
    },
    Xori {
        rs1: usize,
        rd: usize,
        imm: i16
    },
    Ori {
        rs1: usize,
        rd: usize,
        imm: i16
    },
    Andi {
        rs1: usize,
        rd: usize,
        imm: i16
    },
    Lui {
        rd: usize,
        imm: i32
    },
    Auipc {
        rd: usize,
        imm: i32
    },
    Slli {
        shamt: u16,
        rs1: usize,
        rd: usize
    }
}

impl Instructions {
    const OP_IMM: usize = 0b0010011;
    const OP: usize = 0b0110011;
    const LUI: usize = 0b0110111;
    const AUIPC: usize = 0b0010111;

    fn from(bits: usize) -> Option<Instructions> {
        let opcode_mask = 0b1111111;
        let opcode = bits & opcode_mask;
        match opcode {
            Instructions::OP_IMM => {
                let instruction = IFormat::parse(bits);
                match instruction.funct3 {
                    0b000 => Some(AddI {
                        rs1: instruction.rs1,
                        rd: instruction.rd,
                        imm: instruction.imm
                    }),
                    0b001 => Some(Slli {
                        rs1: instruction.rs1,
                        rd: instruction.rd,
                        shamt: instruction.imm as u16
                    }),
                    0b010 => Some(Slti {
                        rs1: instruction.rs1,
                        rd: instruction.rd,
                        imm: instruction.imm
                    }),
                    0b011 => Some(Sltiu {
                        rs1: instruction.rs1,
                        rd: instruction.rd,
                        imm: instruction.imm
                    }),
                    0b100 => Some(Xori {
                        rs1: instruction.rs1,
                        rd: instruction.rd,
                        imm: instruction.imm
                    }),
                    0b110 => Some(Ori {
                        rs1: instruction.rs1,
                        rd: instruction.rd,
                        imm: instruction.imm
                    }),
                    0b111 => Some(Andi {
                        rs1: instruction.rs1,
                        rd: instruction.rd,
                        imm: instruction.imm
                    }),
                    _ => None
                }

            },
            Instructions::OP => {
                let instruction = RFormat::parse(bits);
                let funct = instruction.funct7 << 3 + instruction.funct3;
                match funct {
                    0b0000000000 => Some(Add {
                        rs1: instruction.rs1,
                        rs2: instruction.rs2,
                        rd: instruction.rd
                    }),
                    _ => None
                }
            },
            Instructions::LUI => {
                let instruction = UFormat::parse(bits);
                Some(Lui {
                    rd: instruction.rd,
                    imm: instruction.imm
                })
            },
            Instructions::AUIPC => {
                let instruction = UFormat::parse(bits);
                Some(Auipc {
                    rd: instruction.rd,
                    imm: instruction.imm
                })
            }
            _ => None
        }
    }

    fn execute(self, register: &mut Register) {
        match self {
            AddI { rs1, rd, imm} => {
                let i = register.get(rs1);
                register.put(rd, i + (imm as usize));
            },
            Add { rs1, rs2, rd } => {
                let i = register.get(rs1);
                let j = register.get(rs2);
                register.put(rd, i + j);
            },
            Slti { rs1, rd, imm} => {
                let i = register.get(rs1) as isize;
                if i < (imm as isize) {
                    register.put(rd, 1);
                } else {
                    register.put(rd, 0);
                }
            },
            Sltiu { rs1, rd, imm} => {
                let i = register.get(rs1);
                if i < (imm as usize) {
                    register.put(rd, 1);
                } else {
                    register.put(rd, 0);
                }
            },
            Xori { rs1, rd, imm } => {
                let i = register.get(rs1);
                register.put(rd, i ^ imm);
            },
            Ori {rs1, rd, imm } => {
                let i = register.get(rs1);
                register.put(rd, i | imm);
            },
            Andi {rs1, rd, imm} => {
                let i = register.get(rs1);
                register.put(rd, i & imm);
            },
            Lui {rd, imm} => {
                register.put(rd, (imm as usize) << 12)
            },
            Auipc {rd, imm} => {
                let u_immediate = (imm as usize) << 12;
                register.put(rd, register.pc + u_immediate)
            },
            Slli {shamt, rd, rs1 } => {
                register.put(rd, register.get(rs1) << shamt)
            }
        }
    }

}

struct IFormat {
    imm: i16,
    rs1: usize,
    funct3: usize,
    rd: usize,
    opcode: usize
}

impl IFormat {
    fn parse(bits: usize) -> IFormat {
        let opcode = bits & 0b1111111;
        let rd = bits >> 7 & 0b11111;
        let funct3 = bits >> 12 & 0b111;
        let rs1 = bits >> 15 & 0b11111;
        let imm = (bits >> 20) as i16;
        IFormat {
            imm,
            rs1,
            funct3,
            rd,
            opcode
        }
    }
}

struct RFormat {
    opcode: usize,
    rd: usize,
    funct3: usize,
    rs1: usize,
    rs2: usize,
    funct7: usize
}

impl RFormat {
    fn parse(bits: usize) -> RFormat {
        let opcode = bits & 0b1111111;
        let rd = bits >> 7 & 0b11111;
        let funct3 = bits >> 12 & 0b111;
        let rs1 = bits >> 15 & 0b11111;
        let rs2 = bits >> 20 & 0b11111;
        let funct7 = bits >> 25;
        RFormat {
            opcode,
            rs1,
            rs2,
            funct3,
            funct7,
            rd
        }
    }
}

struct UFormat {
    imm: i32,
    rd: usize,
    opcode: usize
}

impl UFormat {
    fn parse(bits: usize) -> UFormat {
        let opcode = bits & 0b1111111;
        let rd = bits >> 7 & 0b11111;
        let imm = (bits >> 12) as i32;
        UFormat {
            imm,
            rd,
            opcode
        }
    }
}

fn main() {
    let mut register = Register {
        x: [0; 32],
        pc: 0
    };

    let instructions = [
        0b00000000001100000000000010010011,
        0b00000000010100000000000100010011,
        0b00000000000100010000000110110011
    ];

    for instruction in instructions {
        let instruction = Instructions::from(instruction).unwrap();
        println!("{:?}", instruction);

        instruction.execute(&mut register);
        println!("{:?}", register)
    }
}


