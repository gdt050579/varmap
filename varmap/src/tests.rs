use crate::*;
use crate::var_map::Key;
use std::fmt::Debug;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::time::Duration;

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
fn check_value_duration() {
    let d = Duration::new(3, 150_000_000);
    check_type_value(d, ValueKind::Duration(ArenaIndex::new(0, std::mem::size_of::<Duration>() as u32)));

    let mut map = StrVarMap::new();
    map.set("timeout", d);
    assert_eq!(map.get::<Duration>("timeout"), Some(d));
    assert_eq!(map.get_duration("timeout"), Some(d));
    assert!(map.update::<Duration>("timeout", |t| *t += Duration::from_millis(250)));
    assert_eq!(map.get::<Duration>("timeout"), Some(Duration::new(3, 400_000_000)));
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
fn check_update_value_mut() {
    let mut kind = ValueKind::U32(10);
    let mut arena = Arena::new();
    let mut value = ValueMut::view(&mut kind, &mut arena);
    assert!(u32::update(&mut value, |n| *n += 5));
    assert_eq!(kind, ValueKind::U32(15));
}

#[test]
fn check_update_numeric() {
    let mut map = VarMap::new();
    map.set(Key::new(10_000_000), 10u32);
    map.set(Key::new(20_000_000), "text");

    assert!(map.update::<u32>(Key::new(10_000_000), |n| *n += 5));
    assert_eq!(map.get::<u32>(Key::new(10_000_000)), Some(15));

    assert!(!map.update::<u32>(Key::new(20_000_000), |n| *n += 1));
    assert!(!map.update(Key::new(99_000_000), |n: &mut u32| *n += 1));
}

#[test]
fn check_var_map_update_or_default() {
    let mut map = VarMap::new();
    let count = var!("count");
    let name = var!("name");

    let mut called = false;
    assert!(map.update_or_default::<u32>(count, |n| {
        called = true;
        *n += 1;
    }));
    assert!(!called);
    assert_eq!(map.get_u32(count), Some(0));

    assert!(map.update_or_default::<u32>(count, |n| *n += 5));
    assert_eq!(map.get_u32(count), Some(5));

    map.set(name, "alice");
    assert!(!map.update_or_default::<u32>(name, |n| *n += 1));
    assert_eq!(map.get_str(name), Some("alice"));
}

#[test]
fn check_str_var_map_update_or_default() {
    let mut map = StrVarMap::new();

    let mut called = false;
    assert!(map.update_or_default::<u32>("count", |n| {
        called = true;
        *n += 1;
    }));
    assert!(!called);
    assert_eq!(map.get_u32("count"), Some(0));

    assert!(map.update_or_default::<u32>("count", |n| *n += 5));
    assert_eq!(map.get_u32("count"), Some(5));

    map.set("name", "alice");
    assert!(!map.update_or_default::<u32>("name", |n| *n += 1));
    assert_eq!(map.get_str("name"), Some("alice"));
}

#[test]
fn check_enum_var_map_update_or_default() {
    let mut map = EnumVarMap::<TestEnum>::new();

    let mut called = false;
    assert!(map.update_or_default::<u32>(TestEnum::Var1, |n| {
        called = true;
        *n += 1;
    }));
    assert!(!called);
    assert_eq!(map.get_u32(TestEnum::Var1), Some(0));

    assert!(map.update_or_default::<u32>(TestEnum::Var1, |n| *n += 5));
    assert_eq!(map.get_u32(TestEnum::Var1), Some(5));

    map.set(TestEnum::Var2, "alice");
    assert!(map.update_or_default::<u32>(TestEnum::Var2, |n| *n += 1));
    assert_eq!(map.get_str(TestEnum::Var2), Some("alice"));
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

#[derive(EnumVarMap,Copy,Clone,Debug)]
#[repr(u16)]
enum TestEnum {
    Var1,
    Var2,
    Var3,
}

#[test]
fn check_enum_var_map() {
    let mut map = EnumVarMap::<TestEnum>::new();
    map.set(TestEnum::Var1, 1u8);
    map.set(TestEnum::Var2, 2u32);
    map.set(TestEnum::Var3, "Hello, world! ");  
    assert_eq!(map.get_u8(TestEnum::Var1), Some(1u8));
    assert_eq!(map.get_u32(TestEnum::Var2), Some(2u32));
    assert_eq!(map.get_str(TestEnum::Var3), Some("Hello, world! "));
    assert_eq!(map.contains(TestEnum::Var1), true);
    assert_eq!(map.contains(TestEnum::Var2), true);
    assert_eq!(map.contains(TestEnum::Var3), true);
    assert_eq!(map.get_bool(TestEnum::Var1), None);
}

#[test]
fn check_sizes() {
    assert_eq!(std::mem::size_of::<ValueKind>(), 16);
    assert_eq!(std::mem::size_of::<Option<ValueKind>>(), 16);
    assert_eq!(std::mem::size_of::<ArenaIndex>(), 8);
}

#[derive(VarMapValue, Copy, Clone, Eq, PartialEq, Debug)]
struct MyType {
    a: i32,
    b: u32,
    c: u128,
    d: i128,
    e: [u8;3]
}
#[test]
fn check_custom_type() {
    let mut m = StrVarMap::new();
    let obj = MyType { a: 0, b: 1, c: 2, d: 3, e: [0,1,2]};
    m.set("my_type", obj);
    let obj2 = m.get::<MyType>("my_type").unwrap();
    assert_eq!(obj, *obj2);
}

#[test]
fn check_update_custom_type() {
    let mut m = StrVarMap::new();
    let obj = MyType { a: 0, b: 1, c: 2, d: 3, e: [0,1,2]};
    m.set("my_type", obj);
    assert!(m.update::<MyType>("my_type", |obj| obj.a += 1));
    let obj2 = m.get::<MyType>("my_type").unwrap();
    assert_eq!(obj2.a, 1);
}


#[test]
fn check_char() {
    let mut m = StrVarMap::new();
    m.set("char", 'a');
    assert_eq!(m.get_char("char"), Some('a'));
}

#[test]
fn check_ip_addr() {
    let mut m = StrVarMap::new();
    m.set("ip_addr", IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    assert_eq!(m.get_ip("ip_addr"), Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))));
}
#[test]
fn check_ipv4_addr() {
    let mut m = StrVarMap::new();
    m.set("ipv4_addr", Ipv4Addr::new(127, 0, 0, 1));
    assert_eq!(m.get_ipv4("ipv4_addr"), Some(Ipv4Addr::new(127, 0, 0, 1)));
}
#[test]
fn check_ipv6_addr() {
    let mut m = StrVarMap::new();
    m.set("ipv6_addr", Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    assert_eq!(m.get_ipv6("ipv6_addr"), Some(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)));
}
#[test]
fn check_ip_addr_str_var_map() {
    let mut m = StrVarMap::new();
    m.set("ip_addr", IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    assert_eq!(m.get_ip("ip_addr"), Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))));
}
#[test]
fn check_ipv4_addr_str_var_map() {
    let mut m = StrVarMap::new();
    m.set("ipv4_addr", Ipv4Addr::new(127, 0, 0, 1));
    assert_eq!(m.get_ipv4("ipv4_addr"), Some(Ipv4Addr::new(127, 0, 0, 1)));
}

fn pool_payload(len: usize) -> String {
    "x".repeat(len)
}

/// One empty map (no heap) plus mid/large arena payloads. Handles are ordered by `allocated_size`.
fn sized_handles<T>(
    allocate: impl Fn(&mut T) -> PoolHandle,
    set_str: impl Fn(&mut T, PoolHandle, &str),
    size_of: impl Fn(&T, PoolHandle) -> usize,
    pool: &mut T,
) -> ([PoolHandle; 3], [usize; 3]) {
    let small = allocate(pool);

    let mid = allocate(pool);
    let mid_payload = pool_payload(64 * 1024);
    set_str(pool, mid, &mid_payload);

    let large = allocate(pool);
    let large_payload = pool_payload(1024 * 1024);
    set_str(pool, large, &large_payload);

    let mut pairs = [
        (small, size_of(pool, small)),
        (mid, size_of(pool, mid)),
        (large, size_of(pool, large)),
    ];
    pairs.sort_by_key(|(_, sz)| *sz);
    (
        [pairs[0].0, pairs[1].0, pairs[2].0],
        [pairs[0].1, pairs[1].1, pairs[2].1],
    )
}

fn sizes_closest_to_average(sizes: [usize; 3]) -> usize {
    let avg = sizes.iter().sum::<usize>() / sizes.len();
    let min_diff = sizes.iter().map(|s| s.abs_diff(avg)).min().unwrap();
    sizes.into_iter().find(|s| s.abs_diff(avg) == min_diff).unwrap()
}

fn assert_handles_invalid(pool_get: impl Fn(PoolHandle) -> bool, handles: [PoolHandle; 3]) {
    for h in handles {
        assert!(!pool_get(h), "handle should be invalid after clear");
    }
}

fn var_map_pool_three_sizes(pool: &mut VarMapPool) -> ([PoolHandle; 3], [usize; 3]) {
    sized_handles(
        |p| p.allocate(),
        |p, h, s| p.get_mut(h).unwrap().set(var!("s"), s),
        |p, h| p.get(h).unwrap().allocated_size(),
        pool,
    )
}

fn str_var_map_pool_three_sizes(pool: &mut StrVarMapPool) -> ([PoolHandle; 3], [usize; 3]) {
    sized_handles(
        |p| p.allocate(),
        |p, h, s| p.get_mut(h).unwrap().set("s", s),
        |p, h| p.get(h).unwrap().allocated_size(),
        pool,
    )
}

fn enum_var_map_pool_three_sizes(pool: &mut EnumVarMapPool<TestEnum>) -> ([PoolHandle; 3], [usize; 3]) {
    sized_handles(
        |p| p.allocate(),
        |p, h, s| p.get_mut(h).unwrap().set(TestEnum::Var3, s),
        |p, h| p.get(h).unwrap().allocated_size(),
        pool,
    )
}

#[test]
fn check_var_map_pool_clear_strategies() {
    let mut pool = VarMapPool::new();
    pool.clear(ClearStrategy::FreeEntireMemory);
    pool.clear(ClearStrategy::KeepLargestN(1));
    pool.clear(ClearStrategy::KeepSmallestN(1));
    pool.clear(ClearStrategy::KeepNClosestToAverage(1));

    let (handles, sizes) = var_map_pool_three_sizes(&mut pool);
    pool.clear(ClearStrategy::KeepLargestN(1));
    assert_handles_invalid(|h| pool.get(h).is_some(), handles);
    let kept = pool.allocate();
    assert!(pool.get(kept).unwrap().get_str(var!("s")).is_none());
    assert_eq!(pool.get(kept).unwrap().allocated_size(), sizes[2]);
    let fresh = pool.allocate();
    assert!(pool.get(fresh).unwrap().allocated_size() < pool.get(kept).unwrap().allocated_size());
    pool.release(kept);
    pool.release(fresh);

    let (handles, sizes) = var_map_pool_three_sizes(&mut pool);
    pool.clear(ClearStrategy::KeepSmallestN(1));
    assert_handles_invalid(|h| pool.get(h).is_some(), handles);
    let h = pool.allocate();
    assert_eq!(pool.get(h).unwrap().allocated_size(), sizes[0]);
    pool.release(h);

    let (handles, sizes) = var_map_pool_three_sizes(&mut pool);
    pool.clear(ClearStrategy::KeepNClosestToAverage(1));
    assert_handles_invalid(|h| pool.get(h).is_some(), handles);
    let h = pool.allocate();
    assert_eq!(pool.get(h).unwrap().allocated_size(), sizes_closest_to_average(sizes));
    pool.release(h);

    let (handles, sizes) = var_map_pool_three_sizes(&mut pool);
    pool.clear(ClearStrategy::KeepLargestN(2));
    assert_handles_invalid(|h| pool.get(h).is_some(), handles);
    let a = pool.allocate();
    let b = pool.allocate();
    let mut kept = [pool.get(a).unwrap().allocated_size(), pool.get(b).unwrap().allocated_size()];
    let mut expected = [sizes[1], sizes[2]];
    kept.sort();
    expected.sort();
    assert_eq!(kept, expected);
    pool.release(a);
    pool.release(b);

    let (handles, sizes) = var_map_pool_three_sizes(&mut pool);
    pool.clear(ClearStrategy::KeepLargestN(100));
    assert_handles_invalid(|h| pool.get(h).is_some(), handles);
    let a = pool.allocate();
    let b = pool.allocate();
    let c = pool.allocate();
    let mut kept = [
        pool.get(a).unwrap().allocated_size(),
        pool.get(b).unwrap().allocated_size(),
        pool.get(c).unwrap().allocated_size(),
    ];
    kept.sort();
    assert_eq!(kept, sizes);
    pool.release(a);
    pool.release(b);
    pool.release(c);

    let (handles, _) = var_map_pool_three_sizes(&mut pool);
    pool.clear(ClearStrategy::KeepLargestN(0));
    assert_handles_invalid(|h| pool.get(h).is_some(), handles);
    let h = pool.allocate();
    assert!(pool.get(h).unwrap().get_str(var!("s")).is_none());
    pool.release(h);

    let (handles, _) = var_map_pool_three_sizes(&mut pool);
    pool.clear(ClearStrategy::FreeEntireMemory);
    assert_handles_invalid(|h| pool.get(h).is_some(), handles);
    let h = pool.allocate();
    pool.get_mut(h).unwrap().set(var!("n"), 7u8);
    assert!(pool.get(handles[0]).is_none());
    assert_eq!(pool.get(h).unwrap().get_u8(var!("n")), Some(7));
}

#[test]
fn check_str_var_map_pool_clear_strategies() {
    let mut pool = StrVarMapPool::new();

    let (handles, sizes) = str_var_map_pool_three_sizes(&mut pool);
    pool.clear(ClearStrategy::KeepLargestN(1));
    assert_handles_invalid(|h| pool.get(h).is_some(), handles);
    let kept = pool.allocate();
    assert!(!pool.get(kept).unwrap().contains("s"));
    assert_eq!(pool.get(kept).unwrap().allocated_size(), sizes[2]);
    let fresh = pool.allocate();
    assert!(pool.get(fresh).unwrap().allocated_size() < pool.get(kept).unwrap().allocated_size());
    pool.release(kept);
    pool.release(fresh);

    let (handles, sizes) = str_var_map_pool_three_sizes(&mut pool);
    pool.clear(ClearStrategy::KeepSmallestN(1));
    assert_handles_invalid(|h| pool.get(h).is_some(), handles);
    let h = pool.allocate();
    assert_eq!(pool.get(h).unwrap().allocated_size(), sizes[0]);
    pool.release(h);

    let (handles, sizes) = str_var_map_pool_three_sizes(&mut pool);
    pool.clear(ClearStrategy::KeepNClosestToAverage(1));
    assert_handles_invalid(|h| pool.get(h).is_some(), handles);
    let h = pool.allocate();
    assert_eq!(pool.get(h).unwrap().allocated_size(), sizes_closest_to_average(sizes));
    pool.release(h);

    let (handles, sizes) = str_var_map_pool_three_sizes(&mut pool);
    pool.clear(ClearStrategy::KeepLargestN(2));
    assert_handles_invalid(|h| pool.get(h).is_some(), handles);
    let a = pool.allocate();
    let b = pool.allocate();
    let mut kept = [pool.get(a).unwrap().allocated_size(), pool.get(b).unwrap().allocated_size()];
    let mut expected = [sizes[1], sizes[2]];
    kept.sort();
    expected.sort();
    assert_eq!(kept, expected);
    pool.release(a);
    pool.release(b);

    let (handles, _) = str_var_map_pool_three_sizes(&mut pool);
    pool.clear(ClearStrategy::FreeEntireMemory);
    assert_handles_invalid(|h| pool.get(h).is_some(), handles);
    let h = pool.allocate();
    assert!(!pool.get(h).unwrap().contains("s"));
}

#[test]
fn check_enum_var_map_pool_clear_strategies() {
    let mut pool = EnumVarMapPool::<TestEnum>::new();

    let (handles, sizes) = enum_var_map_pool_three_sizes(&mut pool);
    pool.clear(ClearStrategy::KeepLargestN(1));
    assert_handles_invalid(|h| pool.get(h).is_some(), handles);
    let kept = pool.allocate();
    assert!(!pool.get(kept).unwrap().contains(TestEnum::Var3));
    assert_eq!(pool.get(kept).unwrap().allocated_size(), sizes[2]);
    let fresh = pool.allocate();
    assert!(pool.get(fresh).unwrap().allocated_size() < pool.get(kept).unwrap().allocated_size());
    pool.release(kept);
    pool.release(fresh);

    let (handles, sizes) = enum_var_map_pool_three_sizes(&mut pool);
    pool.clear(ClearStrategy::KeepSmallestN(1));
    assert_handles_invalid(|h| pool.get(h).is_some(), handles);
    let h = pool.allocate();
    assert_eq!(pool.get(h).unwrap().allocated_size(), sizes[0]);
    pool.release(h);

    let (handles, sizes) = enum_var_map_pool_three_sizes(&mut pool);
    pool.clear(ClearStrategy::KeepNClosestToAverage(1));
    assert_handles_invalid(|h| pool.get(h).is_some(), handles);
    let h = pool.allocate();
    assert_eq!(pool.get(h).unwrap().allocated_size(), sizes_closest_to_average(sizes));
    pool.release(h);

    let (handles, sizes) = enum_var_map_pool_three_sizes(&mut pool);
    pool.clear(ClearStrategy::KeepLargestN(2));
    assert_handles_invalid(|h| pool.get(h).is_some(), handles);
    let a = pool.allocate();
    let b = pool.allocate();
    let mut kept = [pool.get(a).unwrap().allocated_size(), pool.get(b).unwrap().allocated_size()];
    let mut expected = [sizes[1], sizes[2]];
    kept.sort();
    expected.sort();
    assert_eq!(kept, expected);
    pool.release(a);
    pool.release(b);

    let (handles, _) = enum_var_map_pool_three_sizes(&mut pool);
    pool.clear(ClearStrategy::FreeEntireMemory);
    assert_handles_invalid(|h| pool.get(h).is_some(), handles);
    let h = pool.allocate();
    assert!(!pool.get(h).unwrap().contains(TestEnum::Var1));
    assert!(!pool.get(h).unwrap().contains(TestEnum::Var3));
}

#[test]
fn check_var_map_pool_allocate_release() {
    let mut pool = VarMapPool::new();

    let a = pool.allocate();
    let a_copy = a;
    assert_eq!(a, a_copy);
    pool.get_mut(a).unwrap().set(var!("port"), 8080u16);
    assert_eq!(pool.get(a).unwrap().get_u16(var!("port")), Some(8080));
    assert_eq!(pool.get(a_copy).unwrap().get_u16(var!("port")), Some(8080));

    let b = pool.allocate();
    assert_ne!(a, b);
    pool.get_mut(b).unwrap().set(var!("port"), 443u16);
    assert_eq!(pool.get(a).unwrap().get_u16(var!("port")), Some(8080));
    assert_eq!(pool.get(b).unwrap().get_u16(var!("port")), Some(443));

    pool.release(a);
    assert!(pool.get(a).is_none());
    assert!(pool.get_mut(a_copy).is_none());
    assert_eq!(pool.get(b).unwrap().get_u16(var!("port")), Some(443));

    pool.release(a);
    pool.release(a_copy);

    let c = pool.allocate();
    assert!(pool.get(c).unwrap().get_u16(var!("port")).is_none());
    assert!(pool.get(a).is_none());
    pool.get_mut(c).unwrap().set(var!("host"), "localhost");
    assert_eq!(pool.get(c).unwrap().get_str(var!("host")), Some("localhost"));

    pool.release(b);
    pool.release(c);
    assert!(pool.get(b).is_none());
    assert!(pool.get(c).is_none());
}

#[test]
fn check_str_var_map_pool_allocate_release() {
    let mut pool = StrVarMapPool::new();

    let a = pool.allocate();
    pool.get_mut(a).unwrap().set("name", "alice");
    assert_eq!(pool.get(a).unwrap().get_str("name"), Some("alice"));
    assert!(pool.get(a).unwrap().contains("name"));

    let b = pool.allocate();
    pool.get_mut(b).unwrap().set("name", "bob");
    assert_eq!(pool.get(a).unwrap().get_str("name"), Some("alice"));
    assert_eq!(pool.get(b).unwrap().get_str("name"), Some("bob"));

    pool.release(a);
    assert!(pool.get(a).is_none());
    pool.release(a);

    let c = pool.allocate();
    assert!(!pool.get(c).unwrap().contains("name"));
    assert!(pool.get(a).is_none());

    pool.release(b);
    pool.release(c);
}

#[test]
fn check_enum_var_map_pool_allocate_release() {
    let mut pool = EnumVarMapPool::<TestEnum>::new();

    let a = pool.allocate();
    pool.get_mut(a).unwrap().set(TestEnum::Var1, 1u8);
    assert_eq!(pool.get(a).unwrap().get_u8(TestEnum::Var1), Some(1));

    let b = pool.allocate();
    pool.get_mut(b).unwrap().set(TestEnum::Var2, 2u32);
    assert_eq!(pool.get(a).unwrap().get_u8(TestEnum::Var1), Some(1));
    assert_eq!(pool.get(b).unwrap().get_u32(TestEnum::Var2), Some(2));
    assert!(!pool.get(b).unwrap().contains(TestEnum::Var1));

    pool.release(a);
    assert!(pool.get(a).is_none());
    assert_eq!(pool.get(b).unwrap().get_u32(TestEnum::Var2), Some(2));

    let c = pool.allocate();
    assert!(!pool.get(c).unwrap().contains(TestEnum::Var1));
    assert!(pool.get(a).is_none());

    pool.release(b);
    pool.release(c);
}
