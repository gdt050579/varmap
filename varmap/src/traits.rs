use crate::{Arena, Value, ValueBuilder, ValueKind};

/// Types that can be stored in and read from a varmap.
///
/// Implemented for all built-in storable types and for custom `Copy` types via
/// `#[derive(VarMapValue)]`.
pub trait VarMapValue {
    /// Type returned by [`VarMap::get`](crate::VarMap::get) and related getters.
    type Decoded<'a>;

    /// Type discriminator for custom values (`0` for built-in types).
    const TYPE_ID: u32;

    /// Encodes `self` into a [`Value`] using `builder` for arena-backed payloads.
    fn to_value<'a>(&self, builder: &'a mut ValueBuilder<'a>) -> Value<'a>;

    /// Decodes `self` from a standalone [`Value`].
    fn from_value<'a>(value: &'a Value<'a>) -> Option<Self::Decoded<'a>>;
}

pub(crate) trait VarMapStoredValue: VarMapValue {
    fn from_stored<'a>(kind: &'a ValueKind, arena: &'a Arena) -> Option<Self::Decoded<'a>>;
}

/// Extension of [`VarMapValue`] for custom `Copy` types stored as raw bytes in the arena.
///
/// Implemented automatically by `#[derive(VarMapValue)]`. Manual implementations are only
/// needed when not using the derive macro.
#[doc(hidden)]
pub trait VarMapCustomValue: VarMapValue {
    /// Interprets `bytes` from the arena as this type.
    fn decode_from_bytes<'a>(bytes: &'a [u8]) -> Option<Self::Decoded<'a>>;
}

impl<T: VarMapCustomValue> VarMapStoredValue for T {
    fn from_stored<'a>(kind: &'a ValueKind, arena: &'a Arena) -> Option<T::Decoded<'a>> {
        let ValueKind::Custom(arena_index, type_id) = kind else {
            return None;
        };
        if *type_id != T::TYPE_ID {
            return None;
        }
        let bytes = arena.get(*arena_index)?;
        T::decode_from_bytes(bytes)
    }
}

/// Enum key type for [`EnumVarMap`](struct@crate::EnumVarMap).
///
/// Usually implemented with `#[derive(EnumVarMap)]` rather than by hand.
pub trait EnumVarMapKey: Copy {
    /// Number of enum variants (and slots in the map).
    const INDEX_COUNT: u16;

    /// Index of the slot for this variant.
    fn to_index(self) -> u16;
}
