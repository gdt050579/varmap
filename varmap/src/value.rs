use crate::ArenaIndex;

pub(crate) enum Value {
    Bool(bool),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
    String(ArenaIndex),
    Bytes(ArenaIndex),
    Custom(ArenaIndex),
}
