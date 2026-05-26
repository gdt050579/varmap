use crate::*;

impl VarMapValue for char {
    type Decoded<'a> = char;

    const TYPE_ID: u32 = 0;

    fn to_value<'a>(&self, builder: &'a mut ValueBuilder<'a>) -> Value<'a> {
        Value::new(ValueKind::Char(*self), builder.arena())
    }
    fn from_value<'a>(value: &'a Value<'a>) -> Option<char> {
        <Self as VarMapStoredValue>::from_stored(value.kind(), value.arena())
    }
}

impl VarMapStoredValue for char {
    fn from_stored<'a>(kind: &'a ValueKind, _arena: &'a Arena) -> Option<char> {
        match kind {
            ValueKind::Char(c) => Some(*c),
            _ => None,
        }
    }
}
