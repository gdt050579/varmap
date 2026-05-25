use crate::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Key {
    pub(crate) hash: u64,
}
impl Key {
    #[inline(always)]
    pub fn new(hash: u64) -> Self {
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
    #[inline(always)]
    pub fn contains(&self, key: Key) -> bool {
        let hvalue = key.hash & Hash::HASH_MASK;
        let hash_index = self.hashes.partition_point(|h| h.hash() < hvalue);
        hash_index < self.hashes.len() && self.hashes[hash_index].hash() == hvalue
    }
}