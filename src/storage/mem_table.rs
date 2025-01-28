use crate::core::block::Block;
use crate::core::skip_list::SkipList;
use crate::sys::blocks_ptr;
use crate::sys::pin_memory;

pub struct MemTable {
    pub blocks: SkipList,
    /// stores 10 ringbuffers worth data
    pub last_flushed: i64,
    pub last_compacted: i64,
}
