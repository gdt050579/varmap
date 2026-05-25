use crate::*;

impl VarMapValue for &str {
    type Decoded<'a> = &'a str;
    const TYPE_ID: i32 = 0;

    fn to_value<'a>(&self, builder: &'a mut ValueBuilder<'a>) -> Value<'a> {
        if self.len() <= 14 {
            let mut small_string = [0u8; 14];
            small_string[..self.len()].copy_from_slice(self.as_bytes());
            Value::new(
                ValueKind::SmallString(small_string, self.len() as u8),
                builder.arena(),
            )
        } else {
            Value::new(
                ValueKind::String(builder.arena_mut().store(self.as_bytes(), MemAlign::Bits8)),
                builder.arena(),
            )
        }
    }
    fn from_value<'a>(value: &'a Value<'a>) -> Option<&'a str> {
        match value.kind() {
            ValueKind::String(index) => {
                if let Some(s) = value.arena().get(*index) {
                    Some(unsafe { std::str::from_utf8_unchecked(s) })
                } else {
                    None
                }
            }
            ValueKind::SmallString(small_string, len) => {
                Some(unsafe { std::str::from_utf8_unchecked(&small_string[..*len as usize]) })
            }
            _ => None,
        }
    }
}
