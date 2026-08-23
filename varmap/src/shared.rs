use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// A thread-safe, cheaply-cloneable handle to a `T`, allowing concurrent
/// reads and synchronized (exclusive) writes.
///
/// Backed by `Arc<RwLock<T>>`: cloning bumps a reference count rather than
/// copying the map, so every clone points at the same underlying data. Any
/// number of readers may hold the map at once; a writer gets exclusive access.
///
/// Access goes through [`Shared::read`] / [`Shared::write`], which return lock
/// guards. The guards `Deref` (and `DerefMut`, for the write guard) to `T`, so
/// you call the map's own methods through them.
pub struct Shared<T> {
    inner: Arc<RwLock<T>>,
}

impl<T> Shared<T> {
    /// Wraps `value` in a new shared handle.
    #[inline]
    pub(crate) fn new(value: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(value)),
        }
    }

    /// Acquires shared read access, blocking until no writer holds the lock.
    ///
    /// Multiple readers may hold the returned guard concurrently. The guard
    /// `Deref`s to `T`; the lock is released when it is dropped.
    #[inline]
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        self.inner
            .read()
            .expect("Shared: RwLock poisoned by a panicking writer")
    }

    /// Acquires exclusive write access, blocking until no readers or writer
    /// hold the lock.
    ///
    /// The returned guard `Deref`s and `DerefMut`s to `T`; the lock is released
    /// when it is dropped.
    #[inline]
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.inner
            .write()
            .expect("Shared: RwLock poisoned by a panicking writer")
    }

    /// Attempts to acquire read access without blocking.
    ///
    /// Returns `None` if a writer currently holds the lock.
    #[inline]
    pub fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
        self.inner.try_read().ok()
    }

    /// Attempts to acquire write access without blocking.
    ///
    /// Returns `None` if any reader or a writer currently holds the lock.
    #[inline]
    pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>> {
        self.inner.try_write().ok()
    }

    /// Returns the number of `Shared` handles pointing at this data.
    #[inline]
    pub fn handle_count(&self) -> usize {
        Arc::strong_count(&self.inner)
    }

    /// Consumes this handle and returns the inner `T` if it is the last one.
    ///
    /// Returns `Err(self)` if other handles still exist.
    pub fn try_unwrap(self) -> Result<T, Self> {
        match Arc::try_unwrap(self.inner) {
            Ok(lock) => Ok(lock.into_inner().expect("Shared: RwLock poisoned")),
            Err(inner) => Err(Self { inner }),
        }
    }
}

// Manual `Clone` so `T: Clone` is NOT required — cloning a handle only bumps
// the `Arc` refcount; it never clones `T`.
impl<T> Clone for Shared<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}