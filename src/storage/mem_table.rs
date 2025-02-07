use crate::core::skip_list::{SkipList, SkipListOps};

pub struct MemTable {
     /// stores 10 ringbuffers worth data
    pub blocks: SkipList,
    pub last_flushed: i64,
}
pub trait MemTableOps {
    fn new() -> Self;
    fn add(&mut self, phone_number: &[u8; 16], tombstone_marker: bool) -> bool;
}
impl MemTableOps for MemTable {
    fn new() -> Self {
        MemTable {
            blocks: SkipList::init(),
            last_flushed: 0,
        }
    }
    fn add(&mut self, phone_number: &[u8; 16], tombstone_marker: bool) -> bool {
        return self.blocks.add(phone_number, tombstone_marker);
    }
}
