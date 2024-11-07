use crate::register::Register;
use crate::instruction;
use std::ops::Range;

const SP: usize = 2;

pub struct Processor {
    register: Register,
    memory: [u8; 1024]
}

impl Processor {
    pub fn new() -> Processor {
        let mut proc = Processor {
            register: Register::new(),
            memory: [0u8; 1024]
        };

        // Initialize stack pointer to memory address 256
        proc.set_register_value(SP, 256);
        proc
    }

    pub fn load_instructions(&mut self, executable_code: Vec<u32>) -> () {
        for (idx, instruction) in executable_code.iter().enumerate() {
            self.memory[idx * 4..idx * 4 + 4].copy_from_slice(&instruction.to_be_bytes())
        }
    }

    /// Copies the slice into memory
    pub fn load_into_memory(&mut self, src: &[u8]) -> usize {
        let len = src.len();
        self.memory[512..512 + len].copy_from_slice(src);
        512
    }

    pub fn set_register_value(&mut self, index: usize, value: u32) {
        self.register.put(index, value);
    }

    pub fn execute_instructions(&mut self) {
        println!("--------------------------");
        loop {
            let mut binary= [0u8; 4];
            binary.copy_from_slice(&self.memory[self.register.pc()..self.register.pc() + 4]);
            // println!("[executing] Input: {:0>32b}", binary);
            let instruction = instruction::from(binary).unwrap();
            println!("[executing] Instruction: {:?}", instruction);

            if instruction.should_end(&self.register) {
                break;
            }

            self.register.update_pc(self.register.pc() + 4);

            instruction.execute(&mut self.register, &mut self.memory);
            println!("[executing] Register: {:?}", self.register);
            println!("--------------------------");
        }
    }

    pub fn get_copy_of_memory(&mut self, range: Range<usize>) -> Vec<u8> {
        self.memory[range].to_owned()
    }

    pub fn get_registry_value(&self, index: usize) -> u32 {
        self.register.get(index)
    }
}
