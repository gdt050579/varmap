use crate::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Key {
    pub(crate) hash: u64,
}
impl Key {
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
            #[inline(always)]
            pub fn $name(&self, key: Key) -> Option<$ty> {
                self.get::<$ty>(key)
            }
        )*
    };
}

pub struct VarMap {
    arena: Arena,
    hashes: Vec<Hash>,
    values: Vec<ValueKind>,
}
impl VarMap {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            hashes: Vec::new(),
            values: Vec::new(),
        }
    }
    pub fn clear(&mut self) {
        self.arena.clear();
        self.hashes.clear();
        self.values.clear();
    }
    pub fn set<T: VarMapValue>(&mut self, key: Key, value: T) {
        let mut builder = ValueBuilder::new(&mut self.arena);
        let value_kind = value.to_value(&mut builder).kind().clone();
        let hvalue = key.hash & Hash::HASH_MASK;
        let hash_index = self.hashes.partition_point(|h| h.hash() < hvalue);

        if hash_index < self.hashes.len() && self.hashes[hash_index].hash() == hvalue {
            // overwrite existing value
            let value_index = self.hashes[hash_index].index();
            self.values[value_index] = value_kind;
        } else {
            debug_assert!(self.values.len() < u16::MAX as usize, "Maximum 64k values/keys are supported !");
            // insert new value
            let value_index = self.values.len() as u16;
            self.values.push(value_kind);
            let hash = Hash { data: hvalue | value_index as u64 };
            self.hashes.insert(hash_index, hash);
        }
    }
    #[allow(private_bounds)]
    pub fn get<'a, V: VarMapStoredValue>(&'a self, key: Key) -> Option<V::Decoded<'a>> {
        let hvalue = key.hash & Hash::HASH_MASK;
        let hash_index = self.hashes.partition_point(|h| h.hash() < hvalue);
        if hash_index < self.hashes.len() && self.hashes[hash_index].hash() == hvalue {
            let value_index = self.hashes[hash_index].index();
            let kind = &self.values[value_index];
            V::from_stored(kind, &self.arena)
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
    }

    #[inline(always)]
    pub fn contains(&self, key: Key) -> bool {
        let hvalue = key.hash & Hash::HASH_MASK;
        let hash_index = self.hashes.partition_point(|h| h.hash() < hvalue);
        hash_index < self.hashes.len() && self.hashes[hash_index].hash() == hvalue
    }
}