use crate::*;

impl VarMapValue for &str {
    type Decoded<'a> = &'a str;

    const TYPE_ID: u32 = 0;

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
        <Self as VarMapStoredValue>::from_stored(value.kind(), value.arena())
    }
}

impl VarMapStoredValue for &str {
    fn from_stored<'a>(kind: &'a ValueKind, arena: &'a Arena) -> Option<&'a str> {
        match kind {
            ValueKind::String(index) => arena
                .get(*index)
                .map(|s| unsafe { std::str::from_utf8_unchecked(s) }),
            ValueKind::SmallString(small_string, len) => {
                Some(unsafe { std::str::from_utf8_unchecked(&small_string[..*len as usize]) })
            }
            _ => None,
        }
    }
}
