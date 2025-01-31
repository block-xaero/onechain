use crate::core::skip_list::SkipList;

pub struct MemTable {
    pub blocks: SkipList,
    /// stores 10 ringbuffers worth data
    pub last_flushed: i64,
    pub last_compacted: i64,
}
