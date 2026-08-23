use crate::*;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::time::Duration;

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
///
/// [`VarMap::new`] is `const`, so empty maps can be constructed in `const` / `static` contexts.
///
/// `VarMap` is [`Sync`]. Use [`Self::as_readonly`] to share a read-only view across threads.
pub struct VarMap {
    arena: Arena,
    hashes: Vec<Hash>,
    values: Vec<ValueKind>,
}

impl VarMap {
    /// Creates an empty map.
    ///
    /// This constructor is `const`, so it can be used to initialize maps in `const` or `static`
    /// contexts (for example behind a `Mutex` or other sync wrapper).
    ///
    /// ```
    /// use varmap::VarMap;
    ///
    /// const EMPTY: VarMap = VarMap::new();
    /// ```
    pub const fn new() -> Self {
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

    /// Updates the numeric value at `key` in place when supported for `T`.
    ///
    /// Returns `false` if `key` is missing, the stored type is not `T`, or `T` does not support
    /// in-place updates (see [`VarMapValue::update`]).
    pub fn update<T: VarMapValue>(&mut self, key: Key, f: impl FnOnce(&mut T)) -> bool {
        let hvalue = key.hash & Hash::HASH_MASK;
        let hash_index = self.hashes.partition_point(|h| h.hash() < hvalue);

        let Some(h) = self.hashes.get(hash_index) else {
            return false;
        };
        if h.hash() != hvalue {
            return false;
        }

        let value_index = h.index();
        let kind = &mut self.values[value_index];
        let mut value = ValueMut::view(kind, &mut self.arena);
        T::update(&mut value, f)
    }

    /// Updates the value at `key` in place, or inserts [`Default::default`] if `key` is missing.
    ///
    /// `T` must implement [`Default`].
    ///
    /// - If `key` is missing, `T::default()` is inserted and this returns `true`. `f` is **not**
    ///   called.
    /// - If `key` is present, this is the same as [`Self::update`]: `f` runs when the stored type
    ///   is `T` and `T` supports in-place updates (see [`VarMapValue::update`]).
    ///
    /// Returns `false` if `key` is present but the stored type is not `T`, or `T` does not support
    /// in-place updates.
    ///
    /// ```
    /// use varmap::{var, Key, VarMap};
    ///
    /// let mut map = VarMap::new();
    /// assert!(map.update_or_default::<u32>(var!("count"), |_| {}));
    /// assert_eq!(map.get_u32(var!("count")), Some(0));
    /// assert!(map.update_or_default::<u32>(var!("count"), |n| *n += 1));
    /// assert_eq!(map.get_u32(var!("count")), Some(1));
    /// ```
    pub fn update_or_default<T>(&mut self, key: Key, f: impl FnOnce(&mut T)) -> bool
    where
        T: VarMapValue + Default,
    {
        let hvalue = key.hash & Hash::HASH_MASK;
        let hash_index = self.hashes.partition_point(|h| h.hash() < hvalue);
    
        let value_index = match self.hashes.get(hash_index) {
            Some(h) if h.hash() == hvalue => h.index(),
            _ => {
                self.set(key, T::default());
                return true;
            }
        };
    
        let kind = &mut self.values[value_index];
        let mut value = ValueMut::view(kind, &mut self.arena);
        T::update(&mut value, f)
    }

    /// Returns the value for `key` decoded as `V`.
    ///
    /// Returns `None` if `key` is missing or the stored type does not match `V`.
    pub fn get<'a, V: VarMapValue>(&'a self, key: Key) -> Option<V::Decoded<'a>> {
        let hvalue = key.hash & Hash::HASH_MASK;
        let hash_index = self.hashes.partition_point(|h| h.hash() < hvalue);

        if let Some(h) = self.hashes.get(hash_index) {
            if h.hash() == hvalue {
                let value_index = h.index();
                let kind = &self.values[value_index];
                let value = Value::view(kind, &self.arena);
                V::from_value(&value)
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
        get_duration => Duration,
    }

    /// Returns `true` if `key` has a value (any type).
    #[inline(always)]
    pub fn contains(&self, key: Key) -> bool {
        let hvalue = key.hash & Hash::HASH_MASK;
        let hash_index = self.hashes.partition_point(|h| h.hash() < hvalue);
        hash_index < self.hashes.len() && self.hashes[hash_index].hash() == hvalue
    }

    /// Returns the total allocated size of the map.
    pub fn allocated_size(&self) -> usize {
        self.arena.allocated_size() + self.hashes.capacity() * std::mem::size_of::<Hash>() + self.values.capacity() * std::mem::size_of::<ValueKind>()
    }

    /// Returns a [`Sync`] read-only view of this map.
    ///
    /// The view is `Copy`, `Send`, and `Sync`. Pass copies to other threads for concurrent
    /// reads (getters and [`contains`](Self::contains) only). The borrow checker prevents
    /// mutation of `self` while any view is still alive.
    ///
    /// The view is not `'static`; use [`std::thread::scope`] when spawning. See [`Readonly`].
    #[inline(always)]
    pub fn as_readonly(&self) -> Readonly<'_, Self> {
        Readonly::new(self)
    }
}

impl Default for VarMap {
    fn default() -> Self {
        Self::new()
    }
}
