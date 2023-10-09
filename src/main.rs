use crate::instruction::Instruction;
use crate::register::Register;

mod assembly_compiler;
mod register;
mod instruction;
mod math_utils;

fn main() {
    let mut register = Register::new();
    let mut memory = [0u32; 1024];

    let instructions: Vec<&str> = Vec::from([
        "addi x1, x0, 3",
        "addi x2, x0, 5",
        "add x3, x1, x2"
    ]);

    let instructions: Vec<u32> = assembly_compiler::compile(instructions);

    while register.pc() < instructions.len() {
        let instruction = Instruction::from(instructions[register.pc()]).unwrap();
        println!("{:?}", instruction);

        instruction.execute(&mut register, &mut memory);
        println!("{:?}", register);

        register.update_pc(register.pc() + 1);
    }
}
