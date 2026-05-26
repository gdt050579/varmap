//! Arena alignment helpers used when storing custom values.

/// Alignment bucket for data written into a map arena.
///
/// Used with [`ValueBuilder::build`](crate::ValueBuilder::build) when encoding custom types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MemAlign {
    /// 1-byte alignment.
    Bits8 = 1,
    /// 2-byte alignment.
    Bits16 = 2,
    /// 4-byte alignment.
    Bits32 = 4,
    /// 8-byte alignment.
    Bits64 = 8,
    /// 16-byte alignment.
    Bits128 = 16,
}

impl MemAlign {
    #[inline(always)]
    pub(crate) fn align_offset(&self, offset: usize) -> usize {
        (offset + *self as usize - 1) & !(*self as usize - 1)
    }

    /// Maps a Rust type alignment (1–16 bytes) to the corresponding [`MemAlign`] variant.
    ///
    /// Returns `None` if `align` is zero or greater than 16.
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
