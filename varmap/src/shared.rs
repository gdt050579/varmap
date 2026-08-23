use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// A thread-safe, cheaply cloneable handle to a map (or any `T`).
///
/// Created by [`crate::VarMap::into_shared`], [`crate::StrVarMap::into_shared`], or
/// [`crate::EnumVarMap::into_shared`]. Unlike [`crate::Readonly`], this **owns** the map: clones
/// share one `Arc<RwLock<T>>`, so they are `'static` and can be sent to
/// `thread::spawn` as well as [`std::thread::scope`].
///
/// Any number of readers may hold [`read`](Self::read) at once; [`write`](Self::write)
/// is exclusive. The guards `Deref` (and `DerefMut` for write) to `T`, so you call
/// the map's own getters and setters through them. Drop the guard to release the lock.
///
/// Cloning a handle only bumps the `Arc` count; it does not clone `T`.
/// [`try_unwrap`](Self::try_unwrap) recovers the inner value when this is the last handle.
///
/// A writer that panics poisons the lock. Later `read` / `write` then panic.
///
/// ```
/// use std::thread;
/// use varmap::StrVarMap;
///
/// let mut map = StrVarMap::new();
/// map.set("count", 0u32);
/// let shared = map.into_shared();
///
/// thread::scope(|s| {
///     let a = shared.clone();
///     let b = shared.clone();
///     s.spawn(move || {
///         a.write().update::<u32>("count", |n| *n += 1);
///     });
///     s.spawn(move || {
///         b.write().update::<u32>("count", |n| *n += 1);
///     });
/// });
///
/// assert_eq!(shared.read().get_u32("count"), Some(2));
/// ```
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

/// Cloning a handle shares the same map; `T` is not cloned.
impl<T> Clone for Shared<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}