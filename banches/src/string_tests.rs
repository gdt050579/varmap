use std::collections::{BTreeMap, HashMap};

use crate::data::*;
use varmap::*;

macro_rules! create_test_case {
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

create_test_case! {
    StrVarMapCreateLarge,
    StrVarMap,
    StrVarMap::new(),
    name: "StrVarMap-Update-LargeStrings",
    write: |map, value| map.set(value.var_name, value.large_str_value),
}

create_test_case! {
    VarMapCreateLarge,
    VarMap,
    VarMap::new(),
    name: "VarMap-Update-LargeStrings",
    write: |map, value| map.set(value.var_name_hash, value.large_str_value),
}

create_test_case! {
    EnumVarMapCreateLarge,
    EnumVarMap<TestEnum>,
    EnumVarMap::new(),
    name: "EnumVarMap-Update-LargeStrings",
    write: |map, value| map.set(value.enum_key, value.large_str_value),
}

create_test_case! {
    HashMapCreateLarge,
    HashMap<&'static str, String>,
    HashMap::new(),
    name: "HashMap-Update-LargeStrings",
    write: |map, value| map.insert(value.var_name, value.large_str_value.to_string()),
}

create_test_case! {
    BTreeMapCreateLarge,
    BTreeMap<&'static str, String>,
    BTreeMap::new(),
    name: "BTreeMap-Update-LargeStrings",
    write: |map, value| map.insert(value.var_name, value.large_str_value.to_string()),
}


create_test_case! {
    StrVarMapCreateSmall,
    StrVarMap,
    StrVarMap::new(),
    name: "StrVarMap-Update-SmallStrings",
    write: |map, value| map.set(value.var_name, value.small_str_value),
}

create_test_case! {
    VarMapCreateSmall,
    VarMap,
    VarMap::new(),
    name: "VarMap-Update-SmallStrings",
    write: |map, value| map.set(value.var_name_hash, value.small_str_value),
}

create_test_case! {
    EnumVarMapCreateSmall,
    EnumVarMap<TestEnum>,
    EnumVarMap::new(),
    name: "EnumVarMap-Update-SmallStrings",
    write: |map, value| map.set(value.enum_key, value.small_str_value),
}

create_test_case! {
    HashMapCreateSmall,
    HashMap<&'static str, String>,
    HashMap::new(),
    name: "HashMap-Update-SmallStrings",
    write: |map, value| map.insert(value.var_name, value.small_str_value.to_string()),
}

create_test_case! {
    BTreeMapCreateSmall,
    BTreeMap<&'static str, String>,
    BTreeMap::new(),
    name: "BTreeMap-Update-SmallStrings",
    write: |map, value| map.insert(value.var_name, value.small_str_value.to_string()),
}
