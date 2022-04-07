use crate::instruction::Instruction;
use crate::register::Register;

mod register;
mod instruction;
mod math_utils;

fn main() {
    let mut register = Register::new();

    let instructions = [
        0b00000000001100000000000010010011,
        0b00000000010100000000000100010011,
        0b00000000000100010000000110110011
    ];

    for instruction in instructions {
        let instruction = Instruction::from(instruction).unwrap();
        println!("{:?}", instruction);

        instruction.execute(&mut register);
        println!("{:?}", register)
    }
}


