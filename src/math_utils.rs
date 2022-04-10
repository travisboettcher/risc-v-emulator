use std::u32;

pub trait MixedIntegerOps {
    fn wrapping_add_signed(self, rhs: i32) -> u32;
}

impl MixedIntegerOps for u32 {
    fn wrapping_add_signed(self, rhs: i32) -> u32 {
        if rhs >= 0 {
            u32::wrapping_add(self, rhs as u32)
        } else {
            u32::wrapping_sub(self, rhs.unsigned_abs())
        }
    }
}