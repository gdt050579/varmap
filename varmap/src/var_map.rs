use crate::*;

pub struct VarMap {
    arena: Arena,
    keys: Vec<Key>,
    values: Vec<ValueKind>,
}
impl VarMap {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            keys: Vec::new(),
            values: Vec::new(),
        }
    }
    pub fn set<T: VarMapValue>(&mut self, key: Key, value: T) {
        let mut builder = ValueBuilder::new(&mut self.arena);
        let value_kind = value.to_value(&mut builder).kind().clone();
        self.keys.push(key);
        self.values.push(value_kind);
    }
}