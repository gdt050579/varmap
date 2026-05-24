pub(crate) struct ArenaIndex(u32);
pub(crate) struct Arena {
    data: Vec<u128>,
    current_offset: usize,
}
impl Arena {
    pub(crate) fn new() -> Self {
        Self {
            data: Vec::new(),
            current_offset: 0,
        }
    }
    pub(crate) fn store(&mut self, buf: &[u8], align: u8) -> ArenaIndex {
        let extra = align as usize - 1;
        let start = (self.current_offset + extra) & !extra;
        let end = start + buf.len();
        let data_len = (end + 15) / 16; // rotunjim la 16 bytes
        if data_len > self.data.len() {
            self.data.resize(data_len, 0);
        }
        unsafe {
            let dst = (self.data.as_mut_ptr() as *mut u8).add(start);
            std::ptr::copy_nonoverlapping(buf.as_ptr(), dst, buf.len());
        }
        self.current_offset = end;
        ArenaIndex(start as u32)
    }
    pub(crate) fn clear(&mut self) {
        self.current_offset = 0;
    }
    pub(crate) fn get(&self, index: ArenaIndex) -> Option<&[u8]> {
        let start = index.0 as usize;
        if start >= self.current_offset {
            None
        } else {
            unsafe {
                let p = (self.data.as_ptr() as *const u8).add(start);
                Some(std::slice::from_raw_parts(p, self.current_offset - start))
            }
        }
    }
}
