use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::*;
use crate::var_map::Key;

#[inline(always)]
const fn fnv1a(s: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(0x100000000001b3);
        i += 1;
    }
    hash
}

macro_rules! impl_getters {
    ($($name:ident => $ty:ty),* $(,)?) => {
        $(
            #[doc = concat!("Returns the value as `", stringify!($ty), "`. See [`Self::get`].")]
            #[inline(always)]
            pub fn $name(&self, var_name: &str) -> Option<$ty> {
                self.map.get::<$ty>(Key::new(fnv1a(var_name)))
            }
        )*
    };
}

/// Heterogeneous map keyed by `&str`.
///
/// Each `set` / `get` hashes the name with FNV-1a and delegates to an internal [`VarMap`].
/// For static names, prefer [`VarMap`] with [`var!`](crate::var) for better performance.
pub struct StrVarMap {
    map: VarMap,
}

impl StrVarMap {
    /// Creates an empty map.
    pub fn new() -> Self {
        Self { map: VarMap::new() }
    }

    /// Removes all keys and resets the arena. See [`VarMap::clear`].
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Inserts or overwrites `var_name` with `value`. See [`VarMap::set`].
    pub fn set<T: VarMapValue>(&mut self, var_name: &str, value: T) {
        self.map.set(Key::new(fnv1a(var_name)), value);
    }

    /// Updates the value at `var_name` in place when supported for `T`.
    ///
    /// Returns `false` if `var_name` is missing, the stored type is not `T`, or `T` does not support
    /// in-place updates (see [`VarMapValue::update`]).
    pub fn update<T: VarMapValue>(&mut self, var_name: &str, f: impl FnOnce(&mut T)) -> bool {
        self.map.update(Key::new(fnv1a(var_name)), f)
    }

    /// Returns the value for `var_name` decoded as `V`.
    ///
    /// Returns `None` if the name is missing or the stored type does not match `V`.
    pub fn get<'a, V: VarMapValue>(&'a self, var_name: &str) -> Option<V::Decoded<'a>> {
        self.map.get::<V>(Key::new(fnv1a(var_name)))
    }

    impl_getters! {
        get_bool => bool,
        get_u8   => u8,
        get_u16  => u16,
        get_u32  => u32,
        get_u64  => u64,
        get_i8   => i8,
        get_i16  => i16,
        get_i32  => i32,
        get_i64  => i64,
        get_f32  => f32,
        get_f64  => f64,
        get_str  => &str,
        get_bytes => &[u8],
        get_char => char,
        get_ip => IpAddr,
        get_ipv4 => Ipv4Addr,
        get_ipv6 => Ipv6Addr,
    }

    /// Returns `true` if `var_name` has a value (any type).
    pub fn contains(&self, var_name: &str) -> bool {
        self.map.contains(Key::new(fnv1a(var_name)))
    }
}

impl Default for StrVarMap {
    fn default() -> Self {
        Self::new()
    }
}
