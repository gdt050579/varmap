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
    pub const fn from_align(align: usize) -> Option<Self> {
        match align {
            1 => Some(Self::Bits8),
            2 => Some(Self::Bits16),
            3 | 4 => Some(Self::Bits32),
            5 | 6 | 7 | 8 => Some(Self::Bits64),
            9..=16 => Some(Self::Bits128),
            _ => None,
        }
    }
}
