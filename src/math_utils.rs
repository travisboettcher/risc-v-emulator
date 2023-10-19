use std::u32;

pub trait MixedIntegerOps {
    fn wrapping_add_signed(self, rhs: i32) -> Self;
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

impl MixedIntegerOps for usize {
    fn wrapping_add_signed(self, rhs: i32) -> usize {
        if rhs >= 0 {
            usize::wrapping_add(self, rhs as usize)
        } else {
            usize::wrapping_sub(self, rhs.unsigned_abs() as usize)
        }
    }
}
