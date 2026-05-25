use std::collections::{BTreeMap, HashMap};

use crate::data::*;
use varmap::*;

macro_rules! create_update_test_case {
    (
        $struct:ident,
        $map_ty:ty,
        $new:expr,
        name: $name:literal,
        write: |$map:ident, $value:ident| $body:expr,
    ) => {
        pub struct $struct {
            map: $map_ty,
        }

        impl TestTrait for $struct {
            fn init() -> Self {
                Self { map: $new }
            }

            fn run_test(&mut self, count: usize) {
                let write = |$map: &mut $map_ty, $value: &Data| $body;
                for _ in 0..count {
                    self.map.clear();
                    for value in SMALL_SET {
                        write(&mut self.map, value);
                    }
                }
            }

            const NAME: &'static str = $name;
        }
    };
}

macro_rules! create_read_test_case {
    (
        $struct:ident,
        $map_ty:ty,
        $new:expr,
        name: $name:literal,
        write: |$map:ident, $value:ident| $body:expr,
        read: |$map_read:ident, $value_read:ident| $read_body:expr,
    ) => {
        pub struct $struct {
            map: $map_ty,
        }

        impl TestTrait for $struct {
            fn init() -> Self {
                let mut me = Self { map: $new };
                let write = |$map: &mut $map_ty, $value: &Data| $body;
                for value in SMALL_SET {
                    write(&mut me.map, value);
                }
                me                
            }

            fn run_test(&mut self, count: usize) {
                for _ in 0..count {
                    for value in SMALL_SET {
                        let read = |$map_read: & $map_ty, $value_read: &Data| $read_body;
                        read(&self.map, value);
                    }
                }
            }

            const NAME: &'static str = $name;
        }
    };
}

create_update_test_case! {
    StrVarMapCreateLarge,
    StrVarMap,
    StrVarMap::new(),
    name: "StrVarMap-Update-LargeStrings",
    write: |map, value| map.set(value.var_name, value.large_str_value),
}

create_update_test_case! {
    VarMapCreateLarge,
    VarMap,
    VarMap::new(),
    name: "VarMap-Update-LargeStrings",
    write: |map, value| map.set(value.var_name_hash, value.large_str_value),
}

create_update_test_case! {
    EnumVarMapCreateLarge,
    EnumVarMap<TestEnum>,
    EnumVarMap::new(),
    name: "EnumVarMap-Update-LargeStrings",
    write: |map, value| map.set(value.enum_key, value.large_str_value),
}

create_update_test_case! {
    HashMapCreateLarge,
    HashMap<&'static str, String>,
    HashMap::new(),
    name: "HashMap-Update-LargeStrings",
    write: |map, value| map.insert(value.var_name, value.large_str_value.to_string()),
}

create_update_test_case! {
    BTreeMapCreateLarge,
    BTreeMap<&'static str, String>,
    BTreeMap::new(),
    name: "BTreeMap-Update-LargeStrings",
    write: |map, value| map.insert(value.var_name, value.large_str_value.to_string()),
}


create_update_test_case! {
    StrVarMapCreateSmall,
    StrVarMap,
    StrVarMap::new(),
    name: "StrVarMap-Update-SmallStrings",
    write: |map, value| map.set(value.var_name, value.small_str_value),
}

create_update_test_case! {
    VarMapCreateSmall,
    VarMap,
    VarMap::new(),
    name: "VarMap-Update-SmallStrings",
    write: |map, value| map.set(value.var_name_hash, value.small_str_value),
}

create_update_test_case! {
    EnumVarMapCreateSmall,
    EnumVarMap<TestEnum>,
    EnumVarMap::new(),
    name: "EnumVarMap-Update-SmallStrings",
    write: |map, value| map.set(value.enum_key, value.small_str_value),
}

create_update_test_case! {
    HashMapCreateSmall,
    HashMap<&'static str, String>,
    HashMap::new(),
    name: "HashMap-Update-SmallStrings",
    write: |map, value| map.insert(value.var_name, value.small_str_value.to_string()),
}

create_update_test_case! {
    BTreeMapCreateSmall,
    BTreeMap<&'static str, String>,
    BTreeMap::new(),
    name: "BTreeMap-Update-SmallStrings",
    write: |map, value| map.insert(value.var_name, value.small_str_value.to_string()),
}

// ============================== READ TEST CASES ==============================
create_read_test_case! {
    StrVarMapReadLarge,
    StrVarMap,
    StrVarMap::new(),
    name: "StrVarMap-Read-LargeStrings",
    write: |map, value| map.set(value.var_name, value.large_str_value),
    read: |map, value| { let s = map.get_str(value.var_name); assert_eq!(s.unwrap(), value.large_str_value); },
}

create_read_test_case! {
    VarMapReadLarge,
    VarMap,
    VarMap::new(),
    name: "VarMap-Read-LargeStrings",
    write: |map, value| map.set(value.var_name_hash, value.large_str_value),
    read: |map, value| { let s = map.get_str(value.var_name_hash); assert_eq!(s.unwrap(), value.large_str_value); },
}

create_read_test_case! {
    EnumVarMapReadLarge,
    EnumVarMap<TestEnum>,
    EnumVarMap::new(),
    name: "EnumVarMap-Read-LargeStrings",
    write: |map, value| map.set(value.enum_key, value.large_str_value),
    read: |map, value| { let s = map.get_str(value.enum_key); assert_eq!(s.unwrap(), value.large_str_value); },
}

create_read_test_case! {
    HashMapReadLarge,
    HashMap<&'static str, String>,
    HashMap::new(),
    name: "HashMap-Read-LargeStrings",
    write: |map, value| map.insert(value.var_name, value.large_str_value.to_string()),
    read: |map, value| { let s = map.get(value.var_name); assert_eq!(s.unwrap(), value.large_str_value); },
}

create_read_test_case! {
    BTreeMapReadLarge,
    BTreeMap<&'static str, String>,
    BTreeMap::new(),
    name: "BTreeMap-Read-LargeStrings",
    write: |map, value| map.insert(value.var_name, value.large_str_value.to_string()),
    read: |map, value| { let s = map.get(value.var_name); assert_eq!(s.unwrap(), value.large_str_value); },
}

create_read_test_case! {
    StrVarMapReadSmall,
    StrVarMap,
    StrVarMap::new(),
    name: "StrVarMap-Read-SmallStrings",
    write: |map, value| map.set(value.var_name, value.small_str_value),
    read: |map, value| { let s = map.get_str(value.var_name); assert_eq!(s.unwrap(), value.small_str_value); },
}

create_read_test_case! {
    VarMapReadSmall,
    VarMap,
    VarMap::new(),
    name: "VarMap-Read-SmallStrings",
    write: |map, value| map.set(value.var_name_hash, value.small_str_value),
    read: |map, value| { let s = map.get_str(value.var_name_hash); assert_eq!(s.unwrap(), value.small_str_value); },
}

create_read_test_case! {
    EnumVarMapReadSmall,
    EnumVarMap<TestEnum>,
    EnumVarMap::new(),
    name: "EnumVarMap-Read-SmallStrings",
    write: |map, value| map.set(value.enum_key, value.small_str_value),
    read: |map, value| { let s = map.get_str(value.enum_key); assert_eq!(s.unwrap(), value.small_str_value); },
}

create_read_test_case! {
    HashMapReadSmall,
    HashMap<&'static str, String>,
    HashMap::new(),
    name: "HashMap-Read-SmallStrings",
    write: |map, value| map.insert(value.var_name, value.small_str_value.to_string()),
    read: |map, value| { let s = map.get(value.var_name); assert_eq!(s.unwrap(), value.small_str_value); },
}

create_read_test_case! {
    BTreeMapReadSmall,
    BTreeMap<&'static str, String>,
    BTreeMap::new(),
    name: "BTreeMap-Read-SmallStrings",
    write: |map, value| map.insert(value.var_name, value.small_str_value.to_string()),
    read: |map, value| { let s = map.get(value.var_name); assert_eq!(s.unwrap(), value.small_str_value); },
}