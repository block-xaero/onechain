use crate::core::block::Block;
use crate::sys::blocks_ptr;
use crate::sys::pin_memory;
use rand::Rng;

// 1K blocks logn = 10
const SIZE: usize = 1000;
const HOT_DATA_PROBABILITY_THRESHOLD: f64 = 0.75;
const COLD_DATA_PROBABILITY_THRESHOLD: f64 = 0.25;

pub fn random_level() -> usize {
    let mut level = 1;

    while rand::random::<bool>() && level < 10 {
        level += 1;
    }
    return level;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SkipNode {
    /// bitmap that represents layers this node is present.
    /// 0000 , 0000 (first 4 bits for layer, next 4 bits for offset index on layer 0)
    /// Node is present in layer 0 and offset 1[ 0000, 0001]
    /// Node is present in layer 0 and offset 1 & Node is present in layer 1 and offset 2[ (0000, 0001) , (0001, 0010)]
    pub layer_next: [u8; 10], // 10 layers
    pub tombstone: bool,
    pub data: [u8; 16], // hash of phone number
}
#[repr(C)]
pub struct SkipList {
    // pre-allocate fixed length array of blocks
    pub level0: [Option<SkipNode>; SIZE],
}

pub trait SkipListOps {
    fn add(&mut self, blocks: [u8; 100]) -> bool;
    /// Reader flushes blocks from skip list and writes to SSTable
    fn flush(&mut self) -> bool;

    fn size(&self) -> usize;

    fn search(&self, key: [u8; 100]) -> Option<Block>;
}

impl SkipList {
    fn init() -> SkipList {
        let skip_list = SkipList::_new();
        let blocks_ptr = blocks_ptr(&skip_list);
        // force failure if the memory is not pinned
        pin_memory(blocks_ptr, std::mem::size_of_val(&skip_list.blocks)).unwrap();
        return skip_list;
    }
    fn _new() -> Self {
        return SkipList { level0: [None; 1000] };
    }
}

impl SkipListOps for SkipList {
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
