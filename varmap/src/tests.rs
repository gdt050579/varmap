use crate::*;
use crate::var_map::Key;
use std::fmt::Debug;

fn check_type_value<T>(obj: T, kind: ValueKind)
where
    T: Sized + Copy + PartialEq + Debug + for<'a> VarMapValue<Decoded<'a> = T>,
{
    let mut arena = Arena::new();
    let mut builder = ValueBuilder::new(&mut arena);
    let value = obj.to_value(&mut builder);
    assert_eq!(*value.kind(), kind);
    let value2 = T::from_value(&value);
    assert_eq!(value2, Some(obj));
}
#[test]
fn check_value_bool() {
    check_type_value(true, ValueKind::Bool(true));
    check_type_value(false, ValueKind::Bool(false));
}

#[test]
fn check_value_i8() {
    check_type_value(127i8, ValueKind::I8(127));
    check_type_value(-128i8, ValueKind::I8(-128));
}

#[test]
fn check_value_u8() {
    check_type_value(255u8, ValueKind::U8(255));
    check_type_value(0u8, ValueKind::U8(0));
}

#[test]
fn check_value_i16() {
    check_type_value(32767i16, ValueKind::I16(32767));
    check_type_value(-32768i16, ValueKind::I16(-32768));
}

#[test]
fn check_value_u16() {
    check_type_value(65535u16, ValueKind::U16(65535));
    check_type_value(0u16, ValueKind::U16(0));
}

#[test]
fn check_value_i32() {
    check_type_value(2147483647i32, ValueKind::I32(2147483647));
    check_type_value(-2147483648i32, ValueKind::I32(-2147483648));
}

#[test]
fn check_value_u32() {
    check_type_value(4294967295u32, ValueKind::U32(4294967295));
    check_type_value(0u32, ValueKind::U32(0));
}

#[test]
fn check_value_i64() {
    check_type_value(9223372036854775807i64, ValueKind::I64(9223372036854775807));
    check_type_value(-9223372036854775808i64, ValueKind::I64(-9223372036854775808));
}

#[test]
fn check_value_u64() {
    check_type_value(18446744073709551615u64, ValueKind::U64(18446744073709551615));
    check_type_value(0u64, ValueKind::U64(0));
}

#[test]
fn check_value_f32() {
    check_type_value(3.14f32, ValueKind::F32(3.14f32));
    check_type_value(-3.14f32, ValueKind::F32(-3.14f32));
}

#[test]
fn check_value_f64() {
    check_type_value(3.14f64, ValueKind::F64(3.14f64));
    check_type_value(-3.14f64, ValueKind::F64(-3.14f64));
}

#[test]
fn check_value_i128() {
    let mut arena = Arena::new();
    let mut builder = ValueBuilder::new(&mut arena);
    let value = 9223372036854775807i128.to_value(&mut builder);
    assert_eq!(*value.kind(), ValueKind::I128(ArenaIndex::new(0, 16))); // first offset in the arena index
    let value2 = i128::from_value(&value);
    assert_eq!(value2, Some(9223372036854775807i128));
}

#[test]
fn check_value_u128() {
    let mut arena = Arena::new();
    let mut builder = ValueBuilder::new(&mut arena);
    let value = 18446744073709551615u128.to_value(&mut builder);
    assert_eq!(*value.kind(), ValueKind::U128(ArenaIndex::new(0, 16))); // first offset in the arena index
    let value2 = u128::from_value(&value);
    assert_eq!(value2, Some(18446744073709551615u128));
}

#[test]
fn check_simple() {
    let mut map = VarMap::new();
    map.set(Key::new(10000000), 1u8);
    map.set(Key::new(20000000), 2u32);
    map.set(Key::new(30000000), "Hello, world! ");
    assert_eq!(map.get::<u8>(Key::new(10000000)), Some(1u8));
    assert_eq!(map.get::<u32>(Key::new(20000000)), Some(2u32));
    assert_eq!(map.get::<&str>(Key::new(30000000)), Some("Hello, world! "));
    assert_eq!(map.contains(Key::new(10000000)), true);
    assert_eq!(map.contains(Key::new(20000000)), true);
    assert_eq!(map.contains(Key::new(30000000)), true);
    assert_eq!(map.get::<u8>(Key::new(40000000)), None);
    assert_eq!(map.contains(Key::new(40000000)), false);
    map.set(Key::new(40000000),"Helo");
    let s: &str = map.get::<&str>(Key::new(40000000)).unwrap();
    assert_eq!(s, "Helo");
}

#[test]
fn check_str_var_map() {
    let mut map = StrVarMap::new();
    map.set("var1", 1u8);
    map.set("var2", 2u32);
    map.set("var3", "Hello, world! ");
    assert_eq!(map.get_u8("var1"), Some(1u8));
    assert_eq!(map.get_u32("var2"), Some(2u32));
    assert_eq!(map.get_str("var3"), Some("Hello, world! "));
}

#[test]
fn check_get_bytes() {
    let mut map = StrVarMap::new();
    let bytes = [1u8, 2u8, 3u8];
    map.set("var1", bytes.as_slice());
    assert_eq!(map.get_bytes("var1"), Some(bytes.as_slice()));
}

#[test]
fn check_var_map_var_proc_macro() {
    let mut map = VarMap::new();
    map.set(var!("var1"), 1u8);
    map.set(var!("var2"), 2u32);
    map.set(var!("var3"), "Hello, world! ");
    assert_eq!(map.get_u8(var!("var1")), Some(1u8));
    assert_eq!(map.get_u32(var!("var2")), Some(2u32));
    assert_eq!(map.get_str(var!("var3")), Some("Hello, world! "));
}