use crate::*;

impl VarMapValue for char {
    type Decoded<'a> = char;

    const TYPE_ID: u32 = 0;

    fn to_value<'a>(&self, builder: &'a mut ValueBuilder<'a>) -> Value<'a> {
        Value::new(ValueKind::Char(*self), builder.arena())
    }

    fn from_value<'a>(value: &Value<'a>) -> Option<char> {
        match value.kind() {
            ValueKind::Char(c) => Some(*c),
            _ => None,
        }
    }
}
