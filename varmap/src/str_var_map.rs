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
            #[inline(always)]
            pub fn $name(&self, var_name: &str) -> Option<$ty> {
                self.map.get::<$ty>(Key::new(fnv1a(var_name)))
            }
        )*
    };
}

pub struct StrVarMap {
    map: VarMap,
}
impl StrVarMap {
    pub fn new() -> Self {
        Self { map: VarMap::new() }
    }
    pub fn clear(&mut self) {
        self.map.clear();
    }
    pub fn set<T: VarMapValue>(&mut self, var_name: &str, value: T) {
        self.map.set(Key::new(fnv1a(var_name)), value);
    }
    #[allow(private_bounds)]
    pub fn get<'a, V: VarMapValue>(&'a self, var_name: &str) -> Option<V::Decoded<'a>>
    where
        V: VarMapStoredValue,
    {
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
    }    
    pub fn contains(&self, var_name: &str) -> bool {
        self.map.contains(Key::new(fnv1a(var_name)))
    }
}   

impl Default for StrVarMap {
    fn default() -> Self {
        Self::new()
    }
}