use std::collections::{BTreeMap, HashMap};

use crate::data::*;
use varmap::*;

pub struct StrVarMapCreateLarge {
    map: StrVarMap,
}
impl TestTrait for StrVarMapCreateLarge {
    fn init() -> Self {
        Self {
            map: StrVarMap::new(),
        }
    }
    fn run_test(&mut self, count: usize) {
        for _ in 0..count {
            self.map.clear();
            for value in SMALL_SET {
                self.map.set(value.var_name, value.large_str_value);
            }
        }
    }
    
    const NAME: &'static str = "StrVarMap-Update-LargeStrings";
    const DESCRIPTION: &'static str = "Update large strings in the StrVarMap";
}

pub struct VarMapCreateLarge {
    map: VarMap,
}
impl TestTrait for VarMapCreateLarge {
    fn init() -> Self {
        Self {
            map: VarMap::new(),
        }
    }
    fn run_test(&mut self, count: usize) {
        for _ in 0..count {
            self.map.clear();
            for value in SMALL_SET {
                self.map.set(value.var_name_hash, value.large_str_value);
            }
        }
    }
    
    const NAME: &'static str = "VarMap-Update-LargeStrings";
    const DESCRIPTION: &'static str = "Update large strings in the VarMap";
}

pub struct HashMapCreateLarge {
    map: HashMap<&'static str, String>,
}
impl TestTrait for HashMapCreateLarge {
    fn init() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    fn run_test(&mut self, count: usize) {
        for _ in 0..count {
            self.map.clear();
            for value in SMALL_SET {
                self.map.insert(value.var_name, value.large_str_value.to_string());
            }
        }
    }
    
    const NAME: &'static str = "HashMap-Update-LargeStrings";
    const DESCRIPTION: &'static str = "Update large strings in the HashMap";
}
pub struct BTreeMapCreateLarge {
    map: BTreeMap<&'static str, String>,
}
impl TestTrait for BTreeMapCreateLarge {
    fn init() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }
    fn run_test(&mut self, count: usize) {
        for _ in 0..count {
            self.map.clear();
            for value in SMALL_SET {
                self.map.insert(value.var_name, value.large_str_value.to_string());
            }
        }
    }
    
    const NAME: &'static str = "BTreeMap-Update-LargeStrings";
    const DESCRIPTION: &'static str = "Update large strings in the BTreeMap";
}