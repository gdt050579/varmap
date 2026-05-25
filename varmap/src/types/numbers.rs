use crate::*;

macro_rules! impl_varmap_numeric {
    ($($ty:ty => $variant:ident),* $(,)?) => {
        $(
            impl VarMapValue for $ty {
                type Decoded<'a> = $ty;
                const TYPE_ID: u32 = 0;
                fn to_value<'a>(&self, builder: &'a mut ValueBuilder<'a>) -> Value<'a> {
                    Value::new(ValueKind::$variant(*self), builder.arena())
                }
                fn from_value<'a>(value: &'a Value<'a>) -> Option<$ty> {
                    <$ty as VarMapStoredValue>::from_stored(value.kind(), value.arena())
                }
            }

            impl VarMapStoredValue for $ty {
                fn from_stored<'a>(kind: &'a ValueKind, _arena: &'a Arena) -> Option<$ty> {
                    match kind {
                        ValueKind::$variant(v) => Some(*v),
                        _ => None,
                    }
                }
            }
        )*
    };
}

impl_varmap_numeric! {
    u8   => U8,
    u16  => U16,
    u32  => U32,
    u64  => U64,
    i8   => I8,
    i16  => I16,
    i32  => I32,
    i64  => I64,
    f32  => F32,
    f64  => F64,
}

impl VarMapValue for u128 {
    type Decoded<'a> = u128;

    const TYPE_ID: u32 = 0;

    fn to_value<'a>(&self, builder: &'a mut ValueBuilder<'a>) -> Value<'a> {
        Value::new(
            ValueKind::U128(
                builder
                    .arena_mut()
                    .store(self.to_le_bytes().as_slice(), MemAlign::Bits8),
            ),
            builder.arena(),
        )
    }

    fn from_value<'a>(value: &'a Value<'a>) -> Option<u128> {
        <Self as VarMapStoredValue>::from_stored(value.kind(), value.arena())
    }
}

impl VarMapStoredValue for u128 {
    fn from_stored<'a>(kind: &'a ValueKind, arena: &'a Arena) -> Option<u128> {
        match kind {
            ValueKind::U128(index) => arena
                .get(*index)
                .map(|bytes| u128::from_le_bytes(bytes.try_into().unwrap())),
            _ => None,
        }
    }
}

impl VarMapValue for i128 {
    type Decoded<'a> = i128;

    const TYPE_ID: u32 = 0;

    fn to_value<'a>(&self, builder: &'a mut ValueBuilder<'a>) -> Value<'a> {
        Value::new(
            ValueKind::I128(
                builder
                    .arena_mut()
                    .store(self.to_le_bytes().as_slice(), MemAlign::Bits8),
            ),
            builder.arena(),
        )
    }

    fn from_value<'a>(value: &'a Value<'a>) -> Option<i128> {
        <Self as VarMapStoredValue>::from_stored(value.kind(), value.arena())
    }
}

impl VarMapStoredValue for i128 {
    fn from_stored<'a>(kind: &'a ValueKind, arena: &'a Arena) -> Option<i128> {
        match kind {
            ValueKind::I128(index) => arena
                .get(*index)
                .map(|bytes| i128::from_le_bytes(bytes.try_into().unwrap())),
            _ => None,
        }
    }
}
