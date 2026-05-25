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
            for value in SMALL_SET {
                self.map.set(value.var_name, value.large_str_value);
            }
        }
    }
    
    const NAME: &'static str = "StrMapView-Update-LargeStrings";
    const DESCRIPTION: &'static str = "Update large strings in the StrVarMap";
}
