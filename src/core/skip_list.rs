use crate::sys::pin_memory;
use crate::sys::blocks_ptr;
use crate::storage::ringbuffer::Block;


#[repr(C)]
pub struct SkipList {
    pub size: usize,
    pub capacity: usize,
    pub blocks: [Option<Block>; 1000], // pre-allocate fixed length array of blocks
}

pub trait SkipListOps {
    fn add(&mut self, blocks: [u8; 100]) -> bool;
    /// Reader flushes blocks from skip list and writes to SSTable
    fn flush(&mut self) -> bool;

    fn size(&self) -> usize;

    fn search(&self, key: [u8; 100]) -> Option<Block>;
}

pub impl SkipList {
    
    fn init() -> SkipList {
        let skip_list = SkipList::_new();
        let blocks_ptr = blocks_ptr(&skip_list);
        // force failure if the memory is not pinned
        pin_memory(blocks_ptr, std::mem::size_of_val(&skip_list.blocks)).unwrap();
        return skip_list;
    }
    fn _new() -> Self {
        return SkipList {
            size: 0,
            capacity: 1000,
            blocks: [None; 1000],
        };
    }
}

pub impl SkipListOps for SkipList {
    fn add(&mut self, blocks: [u8; 100]) -> bool {
        todo!()
    }

    fn flush(&mut self) -> bool {
        todo!()
    }

    fn size(&self) -> usize {
        todo!()
    }

    fn search(&self, key: [u8; 100]) -> Option<Block> {
        todo!()
    }
}
