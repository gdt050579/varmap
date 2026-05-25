use crate::*;
use std::marker::PhantomData;

macro_rules! impl_getters {
    ($($name:ident => $ty:ty),* $(,)?) => {
        $(
            #[inline(always)]
            pub fn $name(&self, key: E) -> Option<$ty> {
                self.get::<$ty>(key)
            }
        )*
    };
}

pub struct EnumVarMap<E: EnumVarMapKey> {
    arena: Arena,
    values: Vec<Option<ValueKind>>,
    phantom: PhantomData<E>,
}
impl<E: EnumVarMapKey> EnumVarMap<E> {
    pub fn new() -> Self {
        let mut values = Vec::with_capacity(E::INDEX_COUNT as usize);
        values.resize(E::INDEX_COUNT as usize, None);
        Self {
            arena: Arena::new(),
            values,
            phantom: PhantomData,
        }
    }
    pub fn clear(&mut self) {
        self.arena.clear();
        self.values.clear();
        self.values.resize(E::INDEX_COUNT as usize, None);
    }
    #[inline(always)]
    pub fn set<T: VarMapValue>(&mut self, key: E, value: T) {
        let index = key.to_index() as usize;
        let mut builder = ValueBuilder::new(&mut self.arena);
        let value_kind = *value.to_value(&mut builder).kind();
        self.values[index] = Some(value_kind);
    }
    #[allow(private_bounds)]
    #[inline(always)]
    pub fn get<'a, V: VarMapStoredValue>(&'a self, key: E) -> Option<V::Decoded<'a>> {
        let index = key.to_index() as usize;
        let kind = self.values[index].as_ref()?;
        V::from_stored(kind, &self.arena)        
    }
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
    }    
}

impl<E: EnumVarMapKey> Default for EnumVarMap<E> {
    fn default() -> Self {
        Self::new()
    }
}