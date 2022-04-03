#[derive(Debug)]
pub struct Register {
    x: [usize; 32],
    pc: usize
}

impl Register {
    pub fn new() -> Register {
        Register {
            x: [0; 32],
            pc: 0
        }
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

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
