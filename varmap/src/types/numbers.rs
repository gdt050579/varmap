use crate::*;
macro_rules! impl_varmap_numeric {
    ($($ty:ty => $variant:ident),* $(,)?) => {
        $(
            impl VarMapValue for $ty {
                type Decoded<'a> = $ty;
                const TYPE_ID: i32 = 0;

                fn to_value<'a>(&self, builder: &'a mut ValueBuilder<'a>) -> Value<'a> {
                    Value::new(ValueKind::$variant(*self), builder.arena())
                }

                fn from_value<'a>(value: &'a Value<'a>) -> Option<$ty> {
                    match value.kind() {
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