use crate::*;

impl VarMapValue for &[u8] {
    type Decoded<'a> = &'a [u8];

    fn to_value<'a>(&self, builder: &'a mut ValueBuilder<'a>) -> Value<'a> {
        if self.len() <= 14 {
            let mut small_string = [0u8; 14];
            small_string[..self.len()].copy_from_slice(self);
            Value::new(
                ValueKind::SmallBytes(small_string, self.len() as u8),
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
        match value.kind() {
            ValueKind::Bytes(index) => value.arena().get(*index),
            ValueKind::SmallBytes(small_bytes, len) => {
                Some(&small_bytes[..*len as usize])
            }
            _ => None,
        }
    }
}
