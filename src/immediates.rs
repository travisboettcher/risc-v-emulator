use bitfield::{Bit, BitRange, BitRangeMut};
use immediate_derive::Immediate;

pub trait Immediate {
    fn to_instruction_bitmask(&self) -> u32;
    fn from_instruction(instruction: u32) -> Self;

    fn sign_extend(instruction: u32, lsb: usize) -> u32 {
        let mut i = 0u32;
        let sign = if instruction.bit(31) == true { u32::MAX } else { 0 };
        u32::set_bit_range(&mut i, 31, lsb, sign);
        i
    }
}

struct BitMover<'a> {
    instruction: &'a u32,
    msb: usize,
    lsb: usize
}

impl<'a> BitMover<'a> {
    fn move_bits(instruction: &u32, msb: usize, lsb: usize) -> BitMover {
        BitMover {
            instruction,
            msb,
            lsb
        }
    }
    
    fn to(&self, i: &mut u32, msb: usize, lsb: usize) -> u32 {
        let range: u32 = u32::bit_range(&self.instruction, self.msb, self.lsb);
        u32::set_bit_range(i, msb, lsb, range);
        i.to_owned()
    }
}

#[derive(Debug)]
#[derive(Immediate)]
pub struct IImmediate {
    imm: u32
}
impl Immediate for IImmediate {
    fn to_instruction_bitmask(&self) -> u32 {
        let mut i = 0u32;
        BitMover::move_bits(&self.imm, 11, 0)
            .to(&mut i, 31, 20)
    }

    fn from_instruction(instruction: u32) -> Self {
        let mut i = Self::sign_extend(instruction, 11);
        BitMover::move_bits(&instruction, 30, 20)
            .to(&mut i, 10, 0);
        Self::from(i)
    }
}

#[derive(Debug)]
#[derive(Immediate)]
pub struct SImmediate {
    imm: u32
}

impl Immediate for SImmediate {
    fn to_instruction_bitmask(&self) -> u32 {
        let mut i = 0u32;
        BitMover::move_bits(&self.imm, 4, 0)
            .to(&mut i, 11, 7);
        BitMover::move_bits(&self.imm, 11, 5)
            .to(&mut i, 31, 25)
    }

    fn from_instruction(instruction: u32) -> Self {
        let mut i = Self::sign_extend(instruction, 11);
        BitMover::move_bits(&instruction, 11, 7)
            .to(&mut i, 4, 0);
        BitMover::move_bits(&instruction, 30, 25)
            .to(&mut i, 10, 5);
        Self::from(i)
    }
}

#[derive(Debug)]
#[derive(Immediate)]
pub struct BImmediate {
    imm: u32
}

impl Immediate for BImmediate {
    fn to_instruction_bitmask(&self) -> u32 {
        let mut i = 0u32;
        BitMover::move_bits(&self.imm, 11, 11)
            .to(&mut i, 7, 7);
        BitMover::move_bits(&self.imm, 4, 1)
            .to(&mut i, 11, 8);
        BitMover::move_bits(&self.imm, 10, 5)
            .to(&mut i, 30, 25);
        BitMover::move_bits(&self.imm, 12, 12)
            .to(&mut i, 31, 31)
    }

    fn from_instruction(instruction: u32) -> Self {
        let mut i = Self::sign_extend(instruction, 12);
        BitMover::move_bits(&instruction, 11, 8)
            .to(&mut i, 4, 1);
        BitMover::move_bits(&instruction, 30, 25)
            .to(&mut i, 10, 5);
        BitMover::move_bits(&instruction, 7, 7)
            .to(&mut i, 11, 11);
        Self::from(i)
    }
}

#[derive(Debug)]
#[derive(Immediate)]
pub struct UImmediate {
    imm: u32
}

impl Immediate for UImmediate {
    fn to_instruction_bitmask(&self) -> u32 {
        let mut i = 0u32;
        BitMover::move_bits(&self.imm, 31, 12)
            .to(&mut i, 31, 12)
    }

    fn from_instruction(instruction: u32) -> Self {
        let mut i = 0u32;
        BitMover::move_bits(&instruction, 31, 12)
            .to(&mut i, 31, 12);
        Self::from(i)
    }
}

#[derive(Debug)]
#[derive(Immediate)]
pub struct JImmediate {
    imm: u32
}

impl Immediate for JImmediate {
    fn to_instruction_bitmask(&self) -> u32 {
        let mut i = 0u32;
        BitMover::move_bits(&self.imm, 19, 12)
            .to(&mut i, 19, 12);
        BitMover::move_bits(&self.imm, 11, 11)
            .to(&mut i, 20, 20);
        BitMover::move_bits(&self.imm, 10, 1)
            .to(&mut i, 30, 21);
        BitMover::move_bits(&self.imm, 20, 20)
            .to(&mut i, 31, 31)
    }

    fn from_instruction(instruction: u32) -> Self {
        let mut i = Self::sign_extend(instruction, 20);
        BitMover::move_bits(&instruction, 30, 21)
            .to(&mut i, 10, 1);
        BitMover::move_bits(&instruction, 20, 20)
            .to(&mut i, 11, 11);
        BitMover::move_bits(&instruction, 19, 12)
            .to(&mut i, 19, 12);
        Self::from(i)
    }
}

#[cfg(test)]
mod tests {
    use crate::immediates::{BImmediate, IImmediate, Immediate, JImmediate, SImmediate, UImmediate};

    #[test]
    fn test_i_immediate() {
        let i: IImmediate = 0b001010101010.into();

        assert_eq!(i, IImmediate::from_instruction(i.to_instruction_bitmask()).into());
    }

    #[test]
    fn test_i_immediate_neg() {
        let i: IImmediate = (-0b01010101010i32 as u32).into();

        assert_eq!(i, IImmediate::from_instruction(i.to_instruction_bitmask()).into());
    }

    #[test]
    fn test_s_immediate() {
        let i: SImmediate = 0b001010101010.into();

        assert_eq!(i, SImmediate::from_instruction(i.to_instruction_bitmask()).into());
    }

    #[test]
    fn test_s_immediate_neg() {
        let i: SImmediate = (-0b01010101010i32 as u32).into();

        assert_eq!(i, SImmediate::from_instruction(i.to_instruction_bitmask()).into());
    }

    #[test]
    fn test_b_immediate() {
        let i: SImmediate = 0b001010101010.into();

        assert_eq!(i, SImmediate::from_instruction(i.to_instruction_bitmask()).into());
    }

    #[test]
    fn test_b_immediate_neg() {
        let i: BImmediate = (-0b01010101010i32 as u32).into();

        assert_eq!(i, BImmediate::from_instruction(i.to_instruction_bitmask()).into());
    }

    #[test]
    fn test_u_immediate() {
        let i: UImmediate = (0b001010101010 << 12).into();

        assert_eq!(i, UImmediate::from_instruction(i.to_instruction_bitmask()).into());
    }

    #[test]
    fn test_u_immediate_neg() {
        let i: UImmediate = ((-0b01010101010i32 as u32) << 12).into();

        assert_eq!(i, UImmediate::from_instruction(i.to_instruction_bitmask()).into());
    }

    #[test]
    fn test_j_immediate() {
        let i: JImmediate = 0b001010101010.into();

        assert_eq!(i, JImmediate::from_instruction(i.to_instruction_bitmask()).into());
    }

    #[test]
    fn test_j_immediate_neg() {
        let i: JImmediate = (-0b01010101010i32 as u32).into();

        assert_eq!(i, JImmediate::from_instruction(i.to_instruction_bitmask()).into());
    }

}
