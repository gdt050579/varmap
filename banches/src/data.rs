pub trait TestTrait {
    const NAME: &'static str;
    const DESCRIPTION: &'static str;
    fn init()->Self;
    fn run_test(&mut self, count: usize);
}
