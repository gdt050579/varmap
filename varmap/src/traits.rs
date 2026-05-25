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
