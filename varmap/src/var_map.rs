use crate::*;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// Opaque key for [`VarMap`], holding a 64-bit hash.
///
/// Construct with [`Key::new`] or the [`var!`] macro (FNV-1a of a string literal).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Key {
    pub(crate) hash: u64,
}

impl Key {
    /// Creates a key from a precomputed 64-bit hash.
    #[inline(always)]
    pub const fn new(hash: u64) -> Self {
        Self { hash }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Hash {
    data: u64,
}

impl Hash {
    const HASH_MASK: u64 = 0xFFFF_FFFF_FFFF_0000;
    const INDEX_MASK: u64 = 0x0000_0000_0000_FFFF;
    #[inline(always)]
    fn hash(&self) -> u64 {
        self.data & Hash::HASH_MASK
    }
    #[inline(always)]
    fn index(&self) -> usize {
        (self.data & Hash::INDEX_MASK) as usize
    }
}

macro_rules! impl_getters {
    ($($name:ident => $ty:ty),* $(,)?) => {
        $(
            #[doc = concat!("Returns the value as `", stringify!($ty), "`. See [`Self::get`].")]
            #[inline(always)]
            pub fn $name(&self, key: Key) -> Option<$ty> {
                self.get::<$ty>(key)
            }
        )*
    };
}

/// Heterogeneous map keyed by [`Key`].
///
/// Optimized for compile-time key names via [`var!`]. Supports at most **65 536** distinct keys.
/// See the [crate-level documentation](crate) for the intended write-once-read-many usage model.
pub struct VarMap {
    arena: Arena,
    hashes: Vec<Hash>,
    values: Vec<ValueKind>,
}

impl VarMap {
    /// Creates an empty map.
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            hashes: Vec::new(),
            values: Vec::new(),
        }
    }

    /// Removes all keys and resets the arena offset.
    ///
    /// Clears the hash table and value list; retained heap capacity may be reused on later inserts.
    pub fn clear(&mut self) {
        self.arena.clear();
        self.hashes.clear();
        self.values.clear();
    }

    /// Inserts or overwrites `key` with `value`.
    ///
    /// Overwriting a key does not reclaim arena memory from the previous value. See the
    /// [crate-level documentation](crate) for details.
    pub fn set<T: VarMapValue>(&mut self, key: Key, value: T) {
        let mut builder = ValueBuilder::new(&mut self.arena);
        let value_kind = *value.to_value(&mut builder).kind();
        let hvalue = key.hash & Hash::HASH_MASK;
        let hash_index = self.hashes.partition_point(|h| h.hash() < hvalue);

        if let Some(h) = self.hashes.get(hash_index) {
            if h.hash() == hvalue {
                // overwrite existing value
                let value_index = h.index();
                self.values[value_index] = value_kind;
                return;
            }
        }
        debug_assert!(self.values.len() < u16::MAX as usize, "Maximum 64k values/keys are supported !");
        // insert new value
        let value_index = self.values.len() as u16;
        self.values.push(value_kind);
        let hash = Hash {
            data: hvalue | value_index as u64,
        };
        self.hashes.insert(hash_index, hash);
    }

    /// Returns the value for `key` decoded as `V`.
    ///
    /// Returns `None` if `key` is missing or the stored type does not match `V`.
    #[allow(private_bounds)]
    pub fn get<'a, V: VarMapValue>(&'a self, key: Key) -> Option<V::Decoded<'a>>
    where
        V: VarMapStoredValue,
    {
        let hvalue = key.hash & Hash::HASH_MASK;
        let hash_index = self.hashes.partition_point(|h| h.hash() < hvalue);

        if let Some(h) = self.hashes.get(hash_index) {
            if h.hash() == hvalue {
                let value_index = h.index();
                let kind = &self.values[value_index];
                V::from_stored(kind, &self.arena)
            } else {
                None
            }
        } else {
            None
        }
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

    /// Returns `true` if `key` has a value (any type).
    #[inline(always)]
    pub fn contains(&self, key: Key) -> bool {
        let hvalue = key.hash & Hash::HASH_MASK;
        let hash_index = self.hashes.partition_point(|h| h.hash() < hvalue);
        hash_index < self.hashes.len() && self.hashes[hash_index].hash() == hvalue
    }
}

impl Default for VarMap {
    fn default() -> Self {
        Self::new()
    }
}
