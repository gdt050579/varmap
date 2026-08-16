use crate::{EnumVarMap, EnumVarMapKey, StrVarMap, VarMap};

/// Opaque generational handle to a map checked out from a pool.
///
/// Created by [`VarMapPool::allocate`], [`StrVarMapPool::allocate`], or
/// [`EnumVarMapPool::allocate`]. Copying a handle does not clone the map; every copy
/// refers to the same slot.
///
/// A handle is valid only for the pool that minted it, and only until that slot is
/// [`release`](VarMapPool::release)d or the pool is [`clear`](VarMapPool::clear)ed.
/// Using a stale handle returns `None` or is a no-op.
///
/// The handle type is shared by all three pools and is not tagged with the map type.
/// Do not pass a handle from one pool into another: slot index and generation can
/// coincide, so the other pool may resolve it as a live map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PoolHandle {
    index: u32,
    unique_id: u32,
}

/// Strategy controlling which maps a pool retains when `clear` is called.
///
/// Used by [`VarMapPool::clear`], [`StrVarMapPool::clear`], and [`EnumVarMapPool::clear`].
/// Every strategy invalidates all outstanding [`PoolHandle`]s.
///
/// The `Keep…N` variants keep **`N` maps** (not a byte budget), clamped to the current
/// pool size. Kept maps are cleared so they are empty and reusable; their heap and arena
/// **capacity** is retained. Maps that are not kept are dropped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClearStrategy {
    /// Drop every map and return their memory to the allocator.
    ///
    /// The pool's slot storage is emptied and shrunk. Use this when pooled capacity
    /// should not be retained across a reset.
    FreeEntireMemory,
    /// Keep the `N` maps with the smallest allocated size, drop the rest.
    ///
    /// Kept maps are cleared (empty, capacity retained).
    KeepSmallestN(u32),
    /// Keep the `N` maps whose allocated size is closest to the mean, drop the rest.
    ///
    /// Kept maps are cleared (empty, capacity retained).
    KeepNClosestToAverage(u32),
    /// Keep the `N` maps with the largest allocated size, drop the rest.
    ///
    /// Prefer this when the goal is to retain the most expensive arenas for later
    /// checkouts. Kept maps are cleared (empty, capacity retained).
    KeepLargestN(u32),
}

pub(crate) trait Poolable {
    fn new() -> Self;
    fn clear(&mut self);
    fn allocated_size(&self) -> usize;
}
impl<K: EnumVarMapKey> Poolable for EnumVarMap<K> {
    fn new() -> Self {
        Self::new()
    }
    fn clear(&mut self) {
        self.clear()
    }
    fn allocated_size(&self) -> usize {
        self.allocated_size()
    }
}
impl Poolable for StrVarMap {
    fn new() -> Self {
        Self::new()
    }
    fn clear(&mut self) {
        self.clear()
    }
    fn allocated_size(&self) -> usize {
        self.allocated_size()
    }
}
impl Poolable for VarMap {
    fn new() -> Self {
        Self::new()
    }
    fn clear(&mut self) {
        self.clear()
    }
    fn allocated_size(&self) -> usize {
        self.allocated_size()
    }
}
pub(crate) struct Pool<T: Poolable> {
    objects: Vec<(u32, T)>,
    free_list: Vec<usize>,
    unique_id: u32,
}

impl<T: Poolable> Pool<T> {
    const EMPTY_UNIQUE_ID: u32 = u32::MAX;

    pub(crate) fn new() -> Self {
        Self {
            objects: Vec::new(),
            free_list: Vec::new(),
            unique_id: 0,
        }
    }

    pub(crate) fn allocate(&mut self) -> PoolHandle {
        let unique_id = self.unique_id;
        self.unique_id = (self.unique_id + 1) % Self::EMPTY_UNIQUE_ID;
        if let Some(index) = self.free_list.pop() {
            let entry = &mut self.objects[index];
            entry.0 = unique_id;
            entry.1.clear();
            PoolHandle {
                index: index as u32,
                unique_id,
            }
        } else {
            let index = self.objects.len();
            self.objects.push((unique_id, T::new()));
            PoolHandle {
                index: index as u32,
                unique_id,
            }
        }
    }

    pub(crate) fn release(&mut self, handle: PoolHandle) {
        if let Some(entry) = self.objects.get_mut(handle.index as usize) {
            if entry.0 == handle.unique_id {
                entry.0 = Self::EMPTY_UNIQUE_ID;
                entry.1.clear();
                self.free_list.push(handle.index as usize);
            }
        }
    }

    pub(crate) fn get(&self, handle: PoolHandle) -> Option<&T> {
        self.objects.get(handle.index as usize).filter(|e| e.0 == handle.unique_id).map(|e| &e.1)
    }

    pub(crate) fn get_mut(&mut self, handle: PoolHandle) -> Option<&mut T> {
        self.objects
            .get_mut(handle.index as usize)
            .filter(|e| e.0 == handle.unique_id)
            .map(|e| &mut e.1)
    }

    pub(crate) fn clear(&mut self, strategy: ClearStrategy) {
        self.free_list.clear();
        match strategy {
            ClearStrategy::FreeEntireMemory => {
                self.objects.clear();
                self.objects.shrink_to_fit();
            }
            ClearStrategy::KeepSmallestN(count) => {
                self.objects.sort_by_key(|(_, m)| m.allocated_size());
                self.reset_kept(count);
            }
            ClearStrategy::KeepLargestN(count) => {
                self.objects.sort_by_key(|(_, m)| std::cmp::Reverse(m.allocated_size()));
                self.reset_kept(count);
            }
            ClearStrategy::KeepNClosestToAverage(count) => {
                if self.objects.is_empty() {
                    return;
                }
                let total: usize = self.objects.iter().map(|(_, m)| m.allocated_size()).sum();
                let avg = total / self.objects.len();
                self.objects.sort_by_key(|(_, m)| m.allocated_size().abs_diff(avg));
                self.reset_kept(count);
            }
        }
    }

    /// Keep the first `count` objects, reset them to empty, and rebuild the free list.
    fn reset_kept(&mut self, count: u32) {
        self.objects.truncate(count as usize);
        for (index, entry) in self.objects.iter_mut().enumerate() {
            entry.0 = Self::EMPTY_UNIQUE_ID;
            entry.1.clear();
            self.free_list.push(index);
        }
    }
}

/// Pool of reusable [`VarMap`] values.
///
/// Check out a map with [`allocate`](Self::allocate) and return it with
/// [`release`](Self::release). A released map is cleared but keeps its heap and arena
/// capacity, which is the intended reuse path for this crate's write-once, read-many
/// model.
///
/// Access is through a [`PoolHandle`]. The handle is valid only for this pool and only
/// until the slot is released or the pool is [`clear`](Self::clear)ed.
///
/// ```
/// use varmap::{var, Key, VarMapPool};
///
/// let mut pool = VarMapPool::new();
/// let h = pool.allocate();
/// pool.get_mut(h).unwrap().set(var!("port"), 8080u16);
/// assert_eq!(pool.get(h).unwrap().get_u16(var!("port")), Some(8080));
/// pool.release(h);
/// assert!(pool.get(h).is_none());
/// ```
pub struct VarMapPool {
    pool: Pool<VarMap>,
}
impl VarMapPool {
    /// Creates an empty pool.
    pub fn new() -> Self {
        Self { pool: Pool::new() }
    }

    /// Checks out a [`VarMap`], reusing a released slot when one is available.
    ///
    /// The map is empty. Reused slots are cleared first; previously allocated capacity
    /// is kept. The returned handle is valid until [`release`](Self::release) or
    /// [`clear`](Self::clear).
    pub fn allocate(&mut self) -> PoolHandle {
        self.pool.allocate()
    }

    /// Returns `handle`'s map to the pool.
    ///
    /// The map is cleared and the slot may be reused by a later [`allocate`](Self::allocate).
    /// `handle` and any copies become invalid. A stale handle is ignored.
    pub fn release(&mut self, handle: PoolHandle) {
        self.pool.release(handle)
    }

    /// Returns the [`VarMap`] for `handle`.
    ///
    /// Returns `None` if the handle is stale, was released, or does not refer to a live
    /// slot in this pool.
    pub fn get(&self, handle: PoolHandle) -> Option<&VarMap> {
        self.pool.get(handle)
    }

    /// Returns a mutable [`VarMap`] for `handle`.
    ///
    /// Returns `None` if the handle is stale, was released, or does not refer to a live
    /// slot in this pool.
    pub fn get_mut(&mut self, handle: PoolHandle) -> Option<&mut VarMap> {
        self.pool.get_mut(handle)
    }

    /// Invalidates every outstanding handle and applies `strategy`.
    ///
    /// See [`ClearStrategy`] for what is dropped versus retained. Keep variants leave
    /// kept maps empty while preserving capacity for later [`allocate`](Self::allocate)s.
    pub fn clear(&mut self, strategy: ClearStrategy) {
        self.pool.clear(strategy)
    }
}

/// Pool of reusable [`StrVarMap`] values.
///
/// Same checkout, release, and handle rules as [`VarMapPool`]. Use this when pooled maps
/// are keyed by runtime strings.
///
/// ```
/// use varmap::StrVarMapPool;
///
/// let mut pool = StrVarMapPool::new();
/// let h = pool.allocate();
/// pool.get_mut(h).unwrap().set("host", "localhost");
/// assert_eq!(pool.get(h).unwrap().get_str("host"), Some("localhost"));
/// pool.release(h);
/// ```
pub struct StrVarMapPool {
    pool: Pool<StrVarMap>,
}
impl StrVarMapPool {
    /// Creates an empty pool.
    pub fn new() -> Self {
        Self { pool: Pool::new() }
    }

    /// Checks out a [`StrVarMap`], reusing a released slot when one is available.
    ///
    /// The map is empty. Reused slots are cleared first; previously allocated capacity
    /// is kept. The returned handle is valid until [`release`](Self::release) or
    /// [`clear`](Self::clear).
    pub fn allocate(&mut self) -> PoolHandle {
        self.pool.allocate()
    }

    /// Returns `handle`'s map to the pool.
    ///
    /// The map is cleared and the slot may be reused by a later [`allocate`](Self::allocate).
    /// `handle` and any copies become invalid. A stale handle is ignored.
    pub fn release(&mut self, handle: PoolHandle) {
        self.pool.release(handle)
    }

    /// Returns the [`StrVarMap`] for `handle`.
    ///
    /// Returns `None` if the handle is stale, was released, or does not refer to a live
    /// slot in this pool.
    pub fn get(&self, handle: PoolHandle) -> Option<&StrVarMap> {
        self.pool.get(handle)
    }

    /// Returns a mutable [`StrVarMap`] for `handle`.
    ///
    /// Returns `None` if the handle is stale, was released, or does not refer to a live
    /// slot in this pool.
    pub fn get_mut(&mut self, handle: PoolHandle) -> Option<&mut StrVarMap> {
        self.pool.get_mut(handle)
    }

    /// Invalidates every outstanding handle and applies `strategy`.
    ///
    /// See [`ClearStrategy`] for what is dropped versus retained. Keep variants leave
    /// kept maps empty while preserving capacity for later [`allocate`](Self::allocate)s.
    pub fn clear(&mut self, strategy: ClearStrategy) {
        self.pool.clear(strategy)
    }
}

/// Pool of reusable [`struct@EnumVarMap`] values keyed by `K`.
///
/// Same checkout, release, and handle rules as [`VarMapPool`]. Each checked-out map
/// reserves one slot per variant of `K` (`K::INDEX_COUNT`) up front.
///
/// ```
/// use varmap::{EnumVarMap, EnumVarMapKey, EnumVarMapPool};
///
/// #[derive(EnumVarMap, Copy, Clone, Debug)]
/// #[repr(u16)]
/// enum ConfigKey {
///     Port,
///     Host,
/// }
///
/// let mut pool = EnumVarMapPool::<ConfigKey>::new();
/// let h = pool.allocate();
/// pool.get_mut(h).unwrap().set(ConfigKey::Port, 8080u16);
/// assert_eq!(pool.get(h).unwrap().get_u16(ConfigKey::Port), Some(8080));
/// pool.release(h);
/// ```
pub struct EnumVarMapPool<K: EnumVarMapKey> {
    pool: Pool<EnumVarMap<K>>,
}
impl<K: EnumVarMapKey> EnumVarMapPool<K> {
    /// Creates an empty pool.
    pub fn new() -> Self {
        Self { pool: Pool::new() }
    }

    /// Checks out an [`struct@EnumVarMap`], reusing a released slot when one is available.
    ///
    /// The map is empty (every variant slot unset). Reused slots are cleared first;
    /// previously allocated capacity is kept. The returned handle is valid until
    /// [`release`](Self::release) or [`clear`](Self::clear).
    pub fn allocate(&mut self) -> PoolHandle {
        self.pool.allocate()
    }

    /// Returns `handle`'s map to the pool.
    ///
    /// The map is cleared and the slot may be reused by a later [`allocate`](Self::allocate).
    /// `handle` and any copies become invalid. A stale handle is ignored.
    pub fn release(&mut self, handle: PoolHandle) {
        self.pool.release(handle)
    }

    /// Returns the [`struct@EnumVarMap`] for `handle`.
    ///
    /// Returns `None` if the handle is stale, was released, or does not refer to a live
    /// slot in this pool.
    pub fn get(&self, handle: PoolHandle) -> Option<&EnumVarMap<K>> {
        self.pool.get(handle)
    }

    /// Returns a mutable [`struct@EnumVarMap`] for `handle`.
    ///
    /// Returns `None` if the handle is stale, was released, or does not refer to a live
    /// slot in this pool.
    pub fn get_mut(&mut self, handle: PoolHandle) -> Option<&mut EnumVarMap<K>> {
        self.pool.get_mut(handle)
    }

    /// Invalidates every outstanding handle and applies `strategy`.
    ///
    /// See [`ClearStrategy`] for what is dropped versus retained. Keep variants leave
    /// kept maps empty while preserving capacity for later [`allocate`](Self::allocate)s.
    pub fn clear(&mut self, strategy: ClearStrategy) {
        self.pool.clear(strategy)
    }
}