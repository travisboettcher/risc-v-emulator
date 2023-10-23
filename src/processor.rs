use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use crate::assembly_compiler;
use crate::instruction::{Instruction, JALR};
use crate::register::Register;

const SP: usize = 2;

pub struct Processor {
    register: Register,
    memory: [u32; 1024],
    instruction_index: (usize, usize)
}

impl Processor {
    pub fn new() -> Processor {
        let mut proc = Processor {
            register: Register::new(),
            memory: [0u32; 1024],
            instruction_index: (0, 0)
        };

        // Initialize stack pointer to memory address 256
        proc.set_register_value(SP, 256);
        proc
    }

    pub fn load_instructions(&mut self, file_path: &str) -> (usize, usize) {
        let file = File::open(file_path).expect("no such file");
        let buf = BufReader::new(file);

        let instructions: Vec<String> = buf.lines()
            .flatten()
            .collect();

        let instructions: Vec<u32> = assembly_compiler::compile(instructions);

        self.memory[0..instructions.len()]
            .copy_from_slice(instructions.as_slice());

        self.instruction_index = (0, instructions.len());
        self.instruction_index
    }

    /// Copies the slice into memory
    pub fn load_into_memory(&mut self, src: &[u32]) -> usize {
        let len = src.len();
        self.memory[512..512 + len].copy_from_slice(src);
        512
    }

    pub fn set_register_value(&mut self, index: usize, value: u32) {
        self.register.put(index, value);
    }

    pub fn execute_instructions(&mut self) {
        println!("--------------------------");
        while self.register.pc() / 4 < self.instruction_index.1 {
            let binary = self.memory[self.register.pc() / 4];
            println!("[executing] Input: {:0>32b}", binary);
            let instruction = Instruction::from(binary).unwrap();
            println!("[executing] Instruction: {:?}", instruction);

            if let Instruction::IFormatInstruction { opcode, rd, rs1, ..} = instruction {
                if opcode == JALR && rd == 0 && rs1 == 1 && self.register.get(rs1) == 0 {
                    break;
                }
            }

            self.register.update_pc(self.register.pc() + 4);

            instruction.execute(&mut self.register, &mut self.memory);
            println!("[executing] Register: {:?}", self.register);
            println!("--------------------------");
        }
    }

    pub fn get_copy_of_memory(&mut self, range: Range<usize>) -> Vec<u32> {
        self.memory[range].to_owned()
    }

    pub fn get_registry_value(&self, index: usize) -> u32 {
        self.register.get(index)
    }
}
