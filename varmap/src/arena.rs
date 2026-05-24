use crate::MemAlign;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ArenaIndex {
    offset: u32,
    size: u32,
    typeid: u32,
}
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
    pub(crate) fn store(&mut self, buf: &[u8], align: MemAlign, typeid: u32) -> ArenaIndex {
        let start = align.align_offset(self.current_offset);
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
        ArenaIndex {
            offset: start as u32,
            size: buf.len() as u32,
            typeid,
        }
    }
    pub(crate) fn clear(&mut self) {
        self.current_offset = 0;
    }
    pub(crate) fn get(&self, index: ArenaIndex, typeid: u32) -> Option<&[u8]> {
        if index.typeid != typeid {
            return None;
        }
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
}
