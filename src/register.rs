use crate::error::{EmulatorError, Result};

#[derive(Debug)]
pub struct Register {
    _x: [u32; 32],
    _pc: usize
}

impl Register {
    pub fn new() -> Register {
        Register {
            _x: [0; 32],
            _pc: 0
        }
    }

    pub fn pc(&self) -> usize {
        self._pc
    }

    pub fn update_pc(&mut self, pc: usize) {
        self._pc = pc;
    }

    pub fn put(&mut self, index: usize, value: u32) -> Result<()> {
        if index == 0 {
            return Err(EmulatorError::ModifyZeroRegister);
        }

        if index > 31 {
            return Err(EmulatorError::InvalidRegister(index));
        }

        self._x[index] = value;
        Ok(())
    }

    pub fn get(&self, index: usize) -> Result<u32> {
        if index > 31 {
            return Err(EmulatorError::InvalidRegister(index));
        }

        Ok(self._x[index])
    }

    // Unchecked version for internal use where we know the index is valid
    pub(crate) fn get_unchecked(&self, index: usize) -> u32 {
        self._x[index]
    }

    pub(crate) fn put_unchecked(&mut self, index: usize, value: u32) {
        if index != 0 {
            self._x[index] = value;
        }
    }
}
