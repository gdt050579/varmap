use crate::MemAlign;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ArenaIndex {
    offset: u32,
    size: u32,
}
impl ArenaIndex {
    pub(crate) fn new(offset: u32, size: u32) -> Self {
        Self { offset, size }
    }
}
pub(crate) struct Arena {
    data: Vec<u128>,
    current_offset: usize,
}
impl Arena {
    pub(crate) const fn new() -> Self {
        Self {
            data: Vec::new(),
            current_offset: 0,
        }
    }
    pub(crate) fn store_slice<T: Copy>(&mut self, slice: &[T], align: MemAlign) -> ArenaIndex {
        let bytes = unsafe {
            std::slice::from_raw_parts(
                slice.as_ptr().cast::<u8>(),
                std::mem::size_of_val(slice),
            )
        };
        self.store(bytes, align)
    }

    pub(crate) fn get_slice<T: Copy>(&self, index: ArenaIndex) -> Option<&[T]> {
        let bytes = self.get(index)?;
        let size = std::mem::size_of::<T>();
        if size == 0 || bytes.len() % size != 0 {
            return None;
        }
        debug_assert_eq!(bytes.as_ptr() as usize % std::mem::align_of::<T>(), 0);
        Some(unsafe {
            std::slice::from_raw_parts(bytes.as_ptr().cast::<T>(), bytes.len() / size)
        })
    }

    pub(crate) fn store(&mut self, buf: &[u8], align: MemAlign) -> ArenaIndex {
        let start = align.align_offset(self.current_offset);
        let end = start + buf.len();
        let data_len = end.div_ceil(16); // rotunjim la 16 bytes
        if data_len > self.data.len() {
            self.data.resize(data_len, 0);
        }
        unsafe {
            let dst = (self.data.as_mut_ptr() as *mut u8).add(start);
            std::ptr::copy_nonoverlapping(buf.as_ptr(), dst, buf.len());
        }
        self.current_offset = end;
        ArenaIndex::new(start as u32, buf.len() as u32)
    }
    pub(crate) fn clear(&mut self) {
        self.current_offset = 0;
    }
    pub(crate) fn get(&self, index: ArenaIndex) -> Option<&[u8]> {
        let start = index.offset as usize;
        let end = start + index.size as usize;
        if end > self.current_offset {
            None
        } else {
            unsafe {
                let p = (self.data.as_ptr() as *const u8).add(start);
                Some(std::slice::from_raw_parts(p, index.size as usize))
            }
        }
    }

    pub(crate) fn get_mut(&mut self, index: ArenaIndex) -> Option<&mut [u8]> {
        let start = index.offset as usize;
        let end = start + index.size as usize;
        if end > self.current_offset {
            None
        } else {
            unsafe {
                let p = (self.data.as_mut_ptr() as *mut u8).add(start);
                Some(std::slice::from_raw_parts_mut(p, index.size as usize))
            }
        }
    }

    pub(crate) fn allocated_size(&self) -> usize {
        self.data.capacity() * 16 + std::mem::size_of::<Self>()
    }
}
