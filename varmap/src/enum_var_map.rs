use crate::*;
use std::marker::PhantomData;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

macro_rules! impl_getters {
    ($($name:ident => $ty:ty),* $(,)?) => {
        $(
            #[doc = concat!("Returns the value as `", stringify!($ty), "`. See [`Self::get`].")]
            #[inline(always)]
            pub fn $name(&self, key: E) -> Option<$ty> {
                self.get::<$ty>(key)
            }
        )*
    };
}

/// Heterogeneous map keyed by an enum implementing [`EnumVarMapKey`].
///
/// Each variant has a fixed slot for **O(1)** access without hashing. One slot per variant is
/// reserved at creation (`E::INDEX_COUNT`). See the [crate-level documentation](crate) for usage
/// guidance.
pub struct EnumVarMap<E: EnumVarMapKey> {
    arena: Arena,
    values: Vec<Option<ValueKind>>,
    phantom: PhantomData<E>,
}

impl<E: EnumVarMapKey> EnumVarMap<E> {
    /// Creates a map with one empty slot per enum variant.
    pub fn new() -> Self {
        let mut values = Vec::with_capacity(E::INDEX_COUNT as usize);
        values.resize(E::INDEX_COUNT as usize, None);
        Self {
            arena: Arena::new(),
            values,
            phantom: PhantomData,
        }
    }

    /// Clears every slot and resets the arena offset.
    ///
    /// Slot vector capacity is unchanged; retained arena capacity may be reused.
    pub fn clear(&mut self) {
        self.arena.clear();
        self.values.iter_mut().for_each(|v| *v = None);
    }

    /// Stores `value` in the slot for `key`.
    #[inline(always)]
    pub fn set<T: VarMapValue>(&mut self, key: E, value: T) {
        let index = key.to_index() as usize;
        let mut builder = ValueBuilder::new(&mut self.arena);
        let value_kind = *value.to_value(&mut builder).kind();
        self.values[index] = Some(value_kind);
    }

    /// Updates the value at `key` in place when supported for `T`.
    ///
    /// Returns `false` if `key` is missing, the stored type is not `T`, or `T` does not support
    /// in-place updates (see [`VarMapValue::update`]).
    pub fn update<T: VarMapValue>(&mut self, key: E, f: impl FnOnce(&mut T)) -> bool {
        let index = key.to_index() as usize;
        if let Some(kind) = &mut self.values[index] {
            let mut value = ValueMut::view(kind, &mut self.arena);
            T::update(&mut value, f);
            true
        } else {
            false
        }
    }

    /// Returns the value for `key` decoded as `V`.
    ///
    /// Returns `None` if the slot is empty or the stored type does not match `V`.
    #[inline(always)]
    pub fn get<'a, V: VarMapValue>(&'a self, key: E) -> Option<V::Decoded<'a>> {
        let index = key.to_index() as usize;
        let kind = self.values[index].as_ref()?;
        let value = Value::view(kind, &self.arena);
        V::from_value(&value)
    }

    /// Returns `true` if the slot for `key` has been set.
    #[inline(always)]
    pub fn contains(&self, key: E) -> bool {
        let index = key.to_index() as usize;
        self.values[index].is_some()
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
}

impl<E: EnumVarMapKey> Default for EnumVarMap<E> {
    fn default() -> Self {
        Self::new()
    }
}
