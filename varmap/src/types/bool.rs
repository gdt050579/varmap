use crate::*;

impl VarMapValue for bool {
    type Decoded<'a> = bool;

    const TYPE_ID: u32 = 0;

    fn to_value<'a>(&self, builder: &'a mut ValueBuilder<'a>) -> Value<'a> {
        Value::new(ValueKind::Bool(*self), builder.arena())
    }
    fn from_value<'a>(value: &'a Value<'a>) -> Option<bool> {
        <Self as VarMapStoredValue>::from_stored(value.kind(), value.arena())
    }
}

impl VarMapStoredValue for bool {
    fn from_stored<'a>(kind: &'a ValueKind, _arena: &'a Arena) -> Option<bool> {
        match kind {
            ValueKind::Bool(b) => Some(*b),
            _ => None,
        }
    }
}
