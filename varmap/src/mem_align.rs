#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MemAlign {
    Bits8 = 1,
    Bits16 = 2,
    Bits32 = 4,
    Bits64 = 8,
    Bits128 = 16,
}

impl MemAlign {
    #[inline(always)]
    pub(crate) fn align_offset(&self, offset: usize) -> usize {
        (offset + *self as usize - 1) & !(*self as usize - 1)
    }
}
