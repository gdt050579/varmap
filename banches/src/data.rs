use varmap::*;
pub trait TestTrait {
    const NAME: &'static str;
    const DESCRIPTION: &'static str;
    fn init()->Self;
    fn run_test(&mut self, count: usize);
}

struct Data {
    var_name: &'static str,
    var_name_hash: varmap::Key,
    large_str_value: &'static str,
    small_str_value: &'static str,
    numeric_value: u32,
}
impl Data {
    const fn new(var_name: &'static str, key: varmap::Key, large_str_value: &'static str, small_str_value: &'static str, numeric_value: u32) -> Self {
        Self {
            var_name,
            var_name_hash: key,
            large_str_value,
            small_str_value,
            numeric_value,
        }
    }
}

const SMALL_SET: &[Data] = &[
    Data::new("object.name", var!("object.name"), "John the greatest Rust programmer", "John", 1),
];