use std::net::Ipv4Addr;
use crate::{Arena, ArenaIndex, MemAlign, VarMapValue};

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
    Char(char),
    Ip(ArenaIndex),
    IpV4(Ipv4Addr),
    Ipv6(ArenaIndex),
    SmallString([u8; 14], u8),
    String(ArenaIndex),
    SmallBytes([u8; 14], u8),
    Bytes(ArenaIndex),
    Custom(ArenaIndex, u32),
}

/// An encoded value and its associated arena borrow.
///
/// Produced by [`VarMapValue::to_value`]. Most callers use [`VarMap`](crate::VarMap),
/// [`StrVarMap`](crate::StrVarMap), or [`EnumVarMap`](struct@crate::EnumVarMap) directly instead of
/// handling `Value` explicitly.
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

    /// Returns the raw arena bytes for a custom value of type `T`.
    ///
    /// Returns `None` if this value is not a custom payload, or if `T::TYPE_ID` does not match.
    pub fn as_bytes<T: VarMapValue>(&self) -> Option<&[u8]> {
        match self.kind {
            ValueKind::Custom(arena_index, type_id) => {
                if type_id == T::TYPE_ID {
                    let bytes = self.arena.get(arena_index)?;
                    if bytes.len() != std::mem::size_of::<T>() {
                        return None;
                    }
                    Some(bytes)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

/// Helper for encoding [`VarMapValue`] types into a map arena.
///
/// Created internally when calling [`VarMap::set`](crate::VarMap::set). Custom
/// [`VarMapValue`] implementations use [`build`](Self::build) inside [`VarMapValue::to_value`].
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

    /// Copies `buffer` into the arena with the given alignment and stores it as a custom value.
    ///
    /// `type_id_hash` should be [`VarMapValue::TYPE_ID`] for the type being encoded.
    pub fn build(&mut self, buffer: &[u8], mem_align: MemAlign, type_id_hash: u32) -> Value<'_> {
        let arena_index = self.arena.store(buffer, mem_align);
        Value::new(ValueKind::Custom(arena_index, type_id_hash), self.arena)
    }
}
