use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use crate::assembly_compiler;
use crate::instruction::Instruction;
use crate::register::Register;

pub struct Processor {
    register: Register,
    memory: [u32; 1024],
    instruction_index: (usize, usize)
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            register: Register::new(),
            memory: [0u32; 1024],
            instruction_index: (0, 0)
        }
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
    ///
    /// Uses a dumb algorithm to search for
    /// a segment of memory wide enough to hold the
    /// entire slice
    pub fn load_into_memory(&mut self, src: &[u32]) -> usize {
        let len = src.len();
        let mut i = 0;
        while !self.memory[i..i + len].iter().all(|&m| m == 0) {
            i = i + len;
        }

        self.memory[i..i + len].copy_from_slice(src);

        i
    }

    pub fn set_register_value(&mut self, index: usize, value: u32) {
        self.register.put(index, value);
    }

    pub fn execute_instructions(&mut self) {
        while self.register.pc() / 4 < self.instruction_index.1 {
            let instruction = Instruction::from(self.memory[self.register.pc() / 4]).unwrap();
            println!("{:?}", instruction);

            self.register.update_pc(self.register.pc() + 4);

            instruction.execute(&mut self.register, &mut self.memory);
            println!("{:?}", self.register);
        }
    }

    pub fn get_copy_of_memory(&mut self, range: Range<usize>) -> Vec<u32> {
        self.memory[range].to_owned()
    }

    pub fn get_registry_value(&self, index: usize) -> u32 {
        self.register.get(index)
    }
}
