use crate::register::Register;
use crate::instruction;
use crate::error::{EmulatorError, Result};
use std::ops::Range;

const SP: usize = 2;
const MEMORY_SIZE: usize = 1024;

pub struct Processor {
    register: Register,
    memory: [u8; MEMORY_SIZE]
}

impl Processor {
    pub fn new() -> Processor {
        let mut proc = Processor {
            register: Register::new(),
            memory: [0u8; MEMORY_SIZE]
        };

        // Initialize stack pointer to memory address 256
        // Using put_unchecked since we know SP is valid and not x0
        proc.register.put_unchecked(SP, 256);
        proc
    }

    pub fn load_instructions(&mut self, executable_code: Vec<u32>) -> Result<()> {
        for (idx, instruction) in executable_code.iter().enumerate() {
            let start = idx * 4;
            let end = start + 4;

            if end > MEMORY_SIZE {
                return Err(EmulatorError::MemoryOutOfBounds {
                    address: end,
                    size: MEMORY_SIZE,
                });
            }

            self.memory[start..end].copy_from_slice(&instruction.to_be_bytes());
        }
        Ok(())
    }

    /// Copies the slice into memory starting at address 512
    pub fn load_into_memory(&mut self, src: &[u8]) -> Result<usize> {
        let len = src.len();
        let start = 512;
        let end = start + len;

        if end > MEMORY_SIZE {
            return Err(EmulatorError::MemoryOutOfBounds {
                address: end,
                size: MEMORY_SIZE,
            });
        }

        self.memory[start..end].copy_from_slice(src);
        Ok(start)
    }

    pub fn set_register_value(&mut self, index: usize, value: u32) -> Result<()> {
        self.register.put(index, value)
    }

    pub fn execute_instructions(&mut self) -> Result<()> {
        println!("--------------------------");
        loop {
            let pc = self.register.pc();

            // Check if PC is in bounds
            if pc + 4 > MEMORY_SIZE {
                return Err(EmulatorError::ProgramCounterOutOfBounds {
                    pc,
                    size: MEMORY_SIZE,
                });
            }

            let mut binary = [0u8; 4];
            binary.copy_from_slice(&self.memory[pc..pc + 4]);

            let instruction = instruction::from(binary)?;
            println!("[executing] Instruction: {:?}", instruction);

            if instruction.should_end(&self.register) {
                break;
            }

            self.register.update_pc(pc + 4);

            instruction.execute(&mut self.register, &mut self.memory)?;
            println!("[executing] Register: {:?}", self.register);
            println!("--------------------------");
        }
        Ok(())
    }

    pub fn get_copy_of_memory(&self, range: Range<usize>) -> Result<Vec<u8>> {
        if range.end > MEMORY_SIZE {
            return Err(EmulatorError::MemoryOutOfBounds {
                address: range.end,
                size: MEMORY_SIZE,
            });
        }
        Ok(self.memory[range].to_vec())
    }

    pub fn get_registry_value(&self, index: usize) -> Result<u32> {
        self.register.get(index)
    }
}
