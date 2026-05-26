use crate::{Arena, Value, ValueBuilder, ValueKind};

pub trait VarMapValue {
    type Decoded<'a>;
    const TYPE_ID: u32;

    fn to_value<'a>(&self, builder: &'a mut ValueBuilder<'a>) -> Value<'a>;
    fn from_value<'a>(value: &'a Value<'a>) -> Option<Self::Decoded<'a>>;
}

pub(crate) trait VarMapStoredValue: VarMapValue {
    fn from_stored<'a>(kind: &'a ValueKind, arena: &'a Arena) -> Option<Self::Decoded<'a>>;
}

#[doc(hidden)]
pub trait VarMapCustomValue: VarMapValue {
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

pub trait EnumVarMapKey: Copy {
    const INDEX_COUNT: u16;

    fn to_index(self) -> u16;
}