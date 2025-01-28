use crate::core::sys::pin_memory;
use crate::core::storage::ring_buffer::Block;
use crate::core::sys::blocks_ptr;
use crate::core::skip_list::SkipList;

pub struct MemTable {
    pub blocks: SkipList,
    /// stores 10 ringbuffers worth data
    pub last_flushed: i64,
    pub last_compacted: i64,
}


