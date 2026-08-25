use crate::*;

macro_rules! impl_varmap_slice {
    ($($ty:ty => $variant:ident),* $(,)?) => {
        $(
            impl VarMapValue for &[$ty] {
                type Decoded<'a> = &'a [$ty];
                const TYPE_ID: u32 = 0;

                fn to_value<'a>(&self, builder: &'a mut ValueBuilder<'a>) -> Value<'a> {
                    let align = MemAlign::from_align(std::mem::align_of::<$ty>()).unwrap();
                    let index = builder.arena_mut().store_slice(self, align);
                    Value::new(ValueKind::$variant(index), builder.arena())
                }

                fn from_value<'a>(value: &Value<'a>) -> Option<&'a [$ty]> {
                    match value.kind() {
                        ValueKind::$variant(index) => value.arena().get_slice(*index),
                        _ => None,
                    }
                }
            }
        )*
    };
}

impl_varmap_slice! {
    bool => SliceBool,
    i8  => SliceI8,
    u16 => SliceU16,
    i16 => SliceI16,
    u32 => SliceU32,
    i32 => SliceI32,
    u64 => SliceU64,
    i64 => SliceI64,
    f32 => SliceF32,
    f64 => SliceF64,
}
