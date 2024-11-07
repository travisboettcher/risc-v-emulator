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
