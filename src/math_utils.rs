pub trait MixedIntegerOps {
    fn wrapping_add_signed(self, rhs: isize) -> usize;
}

impl MixedIntegerOps for usize {
    fn wrapping_add_signed(self, rhs: isize) -> usize {
        if rhs >= 0 {
            usize::wrapping_add(self, rhs as usize)
        } else {
            usize::wrapping_sub(self, rhs.unsigned_abs())
        }
    }
}