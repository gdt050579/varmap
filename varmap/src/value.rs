use crate::{Arena, ArenaIndex};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ValueKind {
    Bool(bool),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
    I128(ArenaIndex),
    U128(ArenaIndex),
    SmallString([u8; 14], u8),
    String(ArenaIndex),
    SmallBytes([u8; 14], u8),
    Bytes(ArenaIndex),
    Custom(ArenaIndex, u32),
}

pub struct Value<'a> {
    kind: ValueKind,
    arena: &'a Arena,
}

impl<'a> Value<'a> {
    #[inline(always)]
    pub(crate) fn new(kind: ValueKind, arena: &'a Arena) -> Self {
        Self { kind, arena }
    }
    #[inline(always)]
    pub(crate) fn kind(&self) -> &ValueKind {
        &self.kind
    }
    #[inline(always)]
    pub(crate) fn arena(&self) -> &Arena {
        self.arena
    }
}
pub struct ValueBuilder<'a> {
    arena: &'a mut Arena,
}
impl<'a> ValueBuilder<'a> {
    #[inline(always)]
    pub(crate) fn new(arena: &'a mut Arena) -> Self {
        Self { arena }
    }
    #[inline(always)]
    pub(crate) fn arena(&self) -> &Arena {
        self.arena
    }
    #[inline(always)]
    pub(crate) fn arena_mut(&mut self) -> &mut Arena {
        self.arena
    }
}
