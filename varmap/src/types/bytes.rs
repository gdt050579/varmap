use crate::*;

impl VarMapValue for &[u8] {
    type Decoded<'a> = &'a [u8];

    const TYPE_ID: i32 = 0;

    fn to_value<'a>(&self, builder: &'a mut ValueBuilder<'a>) -> Value<'a> {
        if self.len() <= 14 {
            let mut small_bytes = [0u8; 14];

            small_bytes[..self.len()].copy_from_slice(self);

            Value::new(
                ValueKind::SmallBytes(small_bytes, self.len() as u8),
                builder.arena(),
            )
        } else {
            Value::new(
                ValueKind::Bytes(builder.arena_mut().store(self, MemAlign::Bits8)),
                builder.arena(),
            )
        }
    }

    fn from_value<'a>(value: &'a Value<'a>) -> Option<&'a [u8]> {
        <Self as VarMapStoredValue>::from_stored(value.kind(), value.arena())
    }
}

impl VarMapStoredValue for &[u8] {
    fn from_stored<'a>(kind: &'a ValueKind, arena: &'a Arena) -> Option<&'a [u8]> {
        match kind {
            ValueKind::Bytes(index) => arena.get(*index),
            ValueKind::SmallBytes(small_bytes, len) => Some(&small_bytes[..*len as usize]),
            _ => None,
        }
    }
}
