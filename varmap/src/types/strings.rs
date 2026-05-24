use crate::*;

impl VarMapValue for &str {
    type Decoded<'a> = &'a str;

    fn to_value<'a>(&self, builder: &'a mut ValueBuilder<'a>) -> Value<'a> {
        Value::new(
            ValueKind::String(
                builder
                    .arena_mut()
                    .store(self.as_bytes(), MemAlign::Bits8, 0),
            ),
            builder.arena(),
        )
    }
    fn from_value<'a>(value: &'a Value<'a>) -> Option<&'a str> {
        match value.kind() {
            ValueKind::String(index) => {
                if let Some(s) = value.arena().get(index, 0) {
                    Some(unsafe { std::str::from_utf8_unchecked(s) })
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
