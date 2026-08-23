#[derive(Clone, Copy)]
pub struct Readonly<'a, T: Sync> {
    inner: &'a T,
}

impl<'a, T: Sync> Readonly<'a, T> {
    pub(crate) fn new(map: &'a T) -> Self {
        Self { inner: map }
    }
}

impl<'a, T: Sync> std::ops::Deref for Readonly<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.inner
    }
}