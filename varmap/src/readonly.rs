/// A [`Sync`] read-only view of a map, safe to copy onto other threads.
///
/// [`crate::VarMap`], [`crate::StrVarMap`], and [`struct@crate::EnumVarMap`] are `Send` + `Sync`: every stored
/// value is `Copy` (or a borrow from the arena), and every mutating method takes `&mut self`.
/// Shared `&` access is therefore data-race-free. [`crate::VarMap::as_readonly`] (and the same
/// method on the other map types) wraps that `&self` so the intent is explicit.
///
/// `Readonly` is `Copy`, `Clone`, `Send`, and `Sync`. It implements [`Deref`] to the inner
/// map, so all getters (`get`, `get_u32`, `contains`, …) work as usual. There is no
/// `DerefMut`; writers must use the original map after every view has been dropped.
///
/// The view borrows the map, so it is not `'static`. Use [`std::thread::scope`] (not
/// `thread::spawn`) when sharing it across threads. For concurrent writes as well as
/// reads, consume the map with [`crate::VarMap::into_shared`] (or the same method on the
/// other map types) and use [`crate::Shared`].
///
/// ```
/// use std::thread;
/// use varmap::StrVarMap;
///
/// let mut map = StrVarMap::new();
/// map.set("port", 8080u16);
///
/// let ro = map.as_readonly();
/// thread::scope(|s| {
///     s.spawn(|| assert_eq!(ro.get_u16("port"), Some(8080)));
///     s.spawn(|| assert_eq!(ro.get_u16("port"), Some(8080)));
/// });
/// ```
///
/// [`Deref`]: std::ops::Deref
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

    /// Returns the inner map for shared (read-only) access.
    fn deref(&self) -> &T {
        self.inner
    }
}
