use crate::instruction;

struct BOperation {
    instruction: String,
    source1: String,
    source2: String,
    offset: String
}

struct IOperation {
    instruction: String,
    source: String,
    immediate: String,
    destination: String
}

struct JOperation {
    instruction: String,
    destination: String,
    immediate: String
}

struct ROperation {
    instruction: String,
    source1: String,
    source2: String,
    destination: String
}

struct SOperation {
    instruction: String,
    base: String,
    source: String,
    offset: String
}

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
        let rs1: u32 = self.source1.parse().unwrap();
        let rs2: u32 = self.source2.parse().unwrap();
        let imm: u32 = self.offset.parse().unwrap();
        let op: u32 = match self.instruction.as_str() {
            "beq" => instruction::BEQ,
            "bne" => instruction::BNE,
            "blt" => instruction::BLT,
            "bltu" => instruction::BLTU,
            "bge" => instruction::BGE,
            "bgeu" => instruction::BGEU,
            _ => 0
        };

        instruction::BRANCH
            + (((imm >> 11) & 0b1) << 7)
            + (((imm >> 1) & 0b1111) << 8)
            + (op << 12)
            + (rs1 << 15)
            + (rs2 << 20)
            + (((imm >> 5) & 0b111111) << 25)
            + ((imm >> 12) << 31)
    }
}

impl Operation for IOperation {
    fn compile(self) -> u32 {
        let rs1: u32 = self.source.parse().unwrap();
        let imm: u32 = self.immediate.parse().unwrap();
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
            _ => 0
        };

        match self.instruction.as_str() {
            "jalr" => {
                instruction::JALR
                    + (rd << 7)
                    + (rs1 << 15)
                    + (imm << 20)
            },
            "lw"|"lh"|"lhu"|"lb"|"lbu" => {
                instruction::LOAD
                    + (rd << 7)
                    + (op << 12)
                    + (rs1 << 15)
                    + (imm << 20)
            },
            _ => {
                instruction::OP_IMM
                    + (rd << 7)
                    + ((op & 0b111) << 12)
                    + (rs1 << 15)
                    + (imm << 20)
                    + ((op >> 3) << 25)
            }
        }
    }
}

impl Operation for JOperation {
    fn compile(self) -> u32 {
        let rd: u32 = self.destination.parse().unwrap();
        let imm: u32 = self.immediate.parse().unwrap();
        let op = instruction::JAL;

        op + (rd << 7) + (imm << 12)
    }
}

impl Operation for ROperation {
    fn compile(self) -> u32 {
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
            _ => 0
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
        let rs1: u32 = self.base.parse().unwrap();
        let rs2: u32 = self.source.parse().unwrap();
        let imm: u32 = self.offset.parse().unwrap();
        let width: u32 = match self.instruction.as_str() {
            "sw" => instruction::SW,
            "sh" => instruction::SH,
            "sb" => instruction::SB,
            _ => 0
        };

        instruction::STORE
            + ((imm & 0b11111) << 7)
            + (width << 12)
            + (rs1 << 15)
            + (rs2 << 20)
            + (((imm >> 5) & 0b1111111) << 25)
    }
}

impl Operation for UOperation {
    fn compile(self) -> u32 {
        let rd: u32 = self.destination.parse().unwrap();
        let imm: u32 = self.immediate.parse().unwrap();
        let op: u32 = match self.instruction.as_str() {
            "lui" => instruction::LUI,
            "auipc" => instruction::AUIPC,
            _ => 0
        };

        op + (rd << 7) + (imm << 12)
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

fn compile_line(instruction: &str) -> u32 {
    let tokens = instruction.split_whitespace().collect::<Vec<_>>();
    match tokens[0] {
        token if R_OPS.contains(&token) => {
            ROperation {
                instruction: token.to_owned(),
                destination: parse_register(tokens[1]),
                source1: parse_register(tokens[2]),
                source2: parse_register(tokens[3]),
            }.compile()
        },
        token if I_OPS.contains(&token) => {
            IOperation {
                instruction: token.to_owned(),
                destination: parse_register(tokens[1]),
                source: parse_register(tokens[2]),
                immediate: tokens[3].to_owned(),
            }.compile()
        },
        token if I_OPS_LOAD.contains(&token) => {
            let (offset, base) = parse_base_and_offset(tokens[2]);
            IOperation {
                instruction: token.to_owned(),
                destination: parse_register(tokens[1]),
                source: parse_register(base),
                immediate: offset.to_owned(),
            }.compile()
        },
        token if U_OPS.contains(&token) => {
            UOperation {
                instruction: token.to_owned(),
                destination: parse_register(tokens[1]),
                immediate: tokens[2].to_owned()
            }.compile()
        },
        "jal" => {
            JOperation {
                instruction: tokens[0].to_owned(),
                destination: parse_register(tokens[1]),
                immediate: tokens[2].to_owned()
            }.compile()
        },
        token if B_OPS.contains(&token) => {
            BOperation {
                instruction: token.to_owned(),
                source1: parse_register(tokens[1]),
                source2: parse_register(tokens[2]),
                offset: tokens[3].to_owned()
            }.compile()
        },
        token if S_OPS.contains(&token) => {
            let (offset, base) = parse_base_and_offset(tokens[2]);
            SOperation {
                instruction: token.to_owned(),
                source: parse_register(tokens[1]),
                base: parse_register(base),
                offset: offset.to_owned()
            }.compile()
        },
        _ => 0
    }
}

fn parse_register(token: &str) -> String {
    token
        .trim_start_matches(['x', 't'])
        .trim_end_matches(",")
        .to_owned()
}

fn parse_base_and_offset(token: &str) -> (&str, &str) {
    token.strip_suffix(')')
        .and_then(|c| c.split_once('('))
        .unwrap()
}

pub fn compile(instructions: Vec<&str>) -> Vec<u32> {
    return instructions
        .iter()
        .map(|instruction: &&str| compile_line(instruction))
        .collect();
}

#[cfg(test)]
mod tests {
    use crate::assembly_compiler::compile_line;

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

        assert_eq!(op, 0b00000000010011010010_00101_0110111)
    }

    #[test]
    fn test_compile_jal() {
        let instruction = "jal x5, 1234";

        let op = compile_line(instruction);

        assert_eq!(op, 0b00000000010011010010_00101_1101111)
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

        assert_eq!(op, 0b000000000000_00011_010_00010_0000011)
    }

    #[test]
    fn test_compile_lbu() {
        let instruction = "lbu t2, 0(t3)";

        let op = compile_line(instruction);

        assert_eq!(op, 0b000000000000_00011_100_00010_0000011)
    }

    #[test]
    fn test_compile_sw() {
        let instruction = "sw t2, 0(t3)";

        let op = compile_line(instruction);

        assert_eq!(op, 0b0000000_00010_00011_010_00000_0100011)
    }

}
