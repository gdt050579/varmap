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
                fn from_value<'a>(value: &Value<'a>) -> Option<$ty> {
                    match value.kind() {
                        ValueKind::$variant(v) => Some(*v),
                        _ => None,
                    }
                }
                fn update<F>(value: &mut ValueMut<'_>, f: F) -> bool
                where
                    F: FnOnce(&mut $ty),
                {
                    match value.kind_mut() {
                        ValueKind::$variant(v) => {
                            f(v);
                            true
                        }
                        _ => false,
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

    fn from_value<'a>(value: &Value<'a>) -> Option<u128> {
        match value.kind() {
            ValueKind::U128(index) => value
                .arena()
                .get(*index)
                .map(|bytes| u128::from_le_bytes(bytes.try_into().unwrap())),
            _ => None,
        }
    }

    fn update<F>(value: &mut ValueMut<'_>, f: F) -> bool
    where
        F: FnOnce(&mut u128),
    {
        let index = match *value.kind_mut() {
            ValueKind::U128(index) => index,
            _ => return false,
        };
        let Some(bytes) = value.arena_mut().get_mut(index) else {
            return false;
        };
        if bytes.len() != 16 {
            return false;
        }
        let n = unsafe { &mut *(bytes.as_mut_ptr() as *mut u128) };
        f(n);
        true
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

    fn from_value<'a>(value: &Value<'a>) -> Option<i128> {
        match value.kind() {
            ValueKind::I128(index) => value
                .arena()
                .get(*index)
                .map(|bytes| i128::from_le_bytes(bytes.try_into().unwrap())),
            _ => None,
        }
    }

    fn update<F>(value: &mut ValueMut<'_>, f: F) -> bool
    where
        F: FnOnce(&mut i128),
    {
        let index = match *value.kind_mut() {
            ValueKind::I128(index) => index,
            _ => return false,
        };
        let Some(bytes) = value.arena_mut().get_mut(index) else {
            return false;
        };
        if bytes.len() != 16 {
            return false;
        }
        let n = unsafe { &mut *(bytes.as_mut_ptr() as *mut i128) };
        f(n);
        true
    }
}
