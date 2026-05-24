use crate::*;
impl VarMapValue for bool {
    type Decoded<'a> = bool;

    fn to_value<'a>(&self, builder: &'a mut ValueBuilder<'a>) -> Value<'a> {
        Value::new(ValueKind::Bool(*self), builder.arena())
    }
    fn from_value<'a>(value: &'a Value<'a>) -> Option<bool> {
        match value.kind() {
            ValueKind::Bool(b) => Some(*b),
            _ => None,
        }
    }
}