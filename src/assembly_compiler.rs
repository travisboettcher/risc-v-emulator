use crate::instruction;

struct IOperation {
    instruction: String,
    source: String,
    immediate: String,
    destination: String
}

struct ROperation {
    instruction: String,
    source1: String,
    source2: String,
    destination: String
}

struct UOperation {
    instruction: String,
    destination: String,
    immediate: String
}

trait Operation {
    fn compile(self) -> u32;
}

impl Operation for IOperation {
    fn compile(self) -> u32 {
        let rs1: u32 = self.source.parse().unwrap();
        let imm: u32 = self.immediate.parse().unwrap();
        let rd: u32 = self.destination.parse().unwrap();
        let op: u32 = match self.instruction.as_str() {
            "addi" => instruction::ADDI,
            "sltui" => instruction::SLTIU,
            "andi" => instruction::ANDI,
            "ori" => instruction::ORI,
            "xori" => instruction::XORI,
            "slli" => instruction::SLLI,
            "srli" => instruction::SRLI,
            _ => 0
        };

        instruction::OP_IMM
            + (rd << 7)
            + (op << 12)
            + (rs1 << 15)
            + (imm << 20)
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
            + (op << 12)
            + (rs1 << 15)
            + (rs2 << 20)
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

const I_OPS: &[&str] = &[
    "addi",
    "sltiu",
    "andi",
    "ori",
    "xori",
    "slli",
    "srli",
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
        token if U_OPS.contains(&token) => {
            UOperation {
                instruction: token.to_owned(),
                destination: parse_register(tokens[1]),
                immediate: tokens[2].to_owned()
            }.compile()
        }
        _ => 0
    }
}

fn parse_register(token: &str) -> String {
    token
        .trim_start_matches("x")
        .trim_end_matches(",")
        .to_owned()
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
    fn test_compile_addi() {
        let instruction = "addi x5, x4, 20";

        let op = compile_line(instruction);

        assert_eq!(op, 0b000000010100_00100_000_00101_0010011)
    }

    #[test]
    fn test_compile_lui() {
        let instruction = "lui x5, 1234";

        let op = compile_line(instruction);

        assert_eq!(op, 0b00000000010011010010_00101_0110111)
    }

}
