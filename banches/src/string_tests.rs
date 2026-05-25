use crate::data::*;
use varmap::*;

pub struct StrVarMapSearch {
    map: StrVarMap,
}
impl TestTrait for StrVarMapSearch {
    fn init() -> Self {
        Self {
            map: StrVarMap::new(),
        }
    }
    fn run_test(&mut self, count: usize) {
        for _ in 0..count {
            // for value in LIST {
            //     self.obj.value.clear();
            //     self.obj.value.push_str(value);
            //     assert!(self.c.matches(&self.obj));
            // }
        }
    }
    
    const NAME: &'static str = "String-IsOneOf-1000";
    const DESCRIPTION: &'static str = "Check 1000 words agains a list of 1000 words";
}
