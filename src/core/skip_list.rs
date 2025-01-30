use std::cmp::Ordering;

use crate::core::block::Block;
use crate::datasource::DataSource;
use crate::sys::blocks_ptr;
use crate::sys::pin_memory;

// 1K blocks logn = 10
const MAX_LEVEL: usize = 8;
const CAPACITY: usize = 1000;
const HOT_DATA_PROBABILITY_THRESHOLD: f64 = 0.75;
const MEDIUM_PROBABILITY_THRESHOLD: f64 = 0.50;
const COLD_DATA_PROBABILITY_THRESHOLD: f64 = 0.25;

/// Randomly assign a level to the data provided
/// param data: data to be assigned a level
/// param source: source of data
/// return: level assigned to the data
/// 1. If the data is from ring buffer, bump level with 75% probability
/// 2. If the data is from memtable, bump level with 50% probability
/// 3. If the data is from sstable, bump level with 25% probability
pub fn random_level(data: &[u8; 16], source: DataSource) -> usize {
    let mut level = 1;
    while rand::random::<bool>() && level < MAX_LEVEL {
        match source {
            DataSource::RingBuffer => {
                if rand::random::<f64>() > HOT_DATA_PROBABILITY_THRESHOLD {
                    level += 1;
                }
            }
            DataSource::MemTable => {
                if rand::random::<f64>() > MEDIUM_PROBABILITY_THRESHOLD {
                    level += 1;
                }
            }

            DataSource::SSTable => {
                if rand::random::<f64>() > COLD_DATA_PROBABILITY_THRESHOLD {
                    level += 1;
                }
            }
        }
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
    pub layer_next: [u8; MAX_LEVEL], // 10 layers
    pub tombstone: bool,
    pub data: [u8; 16], // hash of phone number
}
impl PartialEq for SkipNode {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl Eq for SkipNode {}
impl PartialOrd for SkipNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.data.cmp(&other.data)) // Lexicographic comparison
    }
}

impl Ord for SkipNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data.cmp(&other.data)
    }
}
impl SkipNode {
    fn new(data: [u8; 16], tombstone_marker: bool, source: DataSource) -> Self {
        return SkipNode {
            layer_next: [0; 8],
            tombstone: tombstone_marker,
            data,
        };
    }
}
#[repr(C)]
pub struct SkipList {
    /// represents a sorted array of
    // heads at each level of skip list
    // levels are fixed
    pub heads: [usize; MAX_LEVEL],
    // pre-allocate fixed length array of blocks
    pub blocks: [Option<SkipNode>; CAPACITY],
}

pub trait SkipListOps {
    fn add(&mut self, data: &[u8; 16], tombstone_marker: bool) -> bool;
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
        return SkipList {
            heads: [usize::MAX; 8],
            blocks: [None; 1000],
        };
    }
}

impl SkipListOps for SkipList {
    fn add(&mut self, data: &[u8; 16], tombstone_marker: bool) -> bool {
        let mut inserted = false;
        if self.size() == CAPACITY {
            // flush the skip list to SSTable
            inserted = self.flush();
        } else {
            let max_level = random_level(&data, DataSource::RingBuffer);
            let mut new_node = SkipNode::new(*data, tombstone_marker, DataSource::RingBuffer);
            // find right place to insert node
            for i in (0..max_level).rev() {
                let mut pos = self.heads[i];
                while pos < self.blocks.len()
                    && self.blocks[pos].is_some()
                    && self.blocks[pos].unwrap().data < new_node.data
                {
                    pos += 1;
                }
                if pos < self.blocks.len()
                    && self.blocks[pos].is_some()
                    && (self.blocks[pos].unwrap().data >= new_node.data)
                {
                    // insert at this level
                    let byte_offset = i / 8;
                    let bit_offset = i % 4;
                    let step_offset = (pos - self.heads[i]) % 4;
                    let higher_nibble =
                        (new_node.layer_next[byte_offset] & 0xF0) | 1 << (bit_offset + 4);
                    let lower_nibble =
                        (new_node.layer_next[byte_offset] & 0x0F) | (1 << step_offset);
                    new_node.layer_next[byte_offset] = higher_nibble | lower_nibble;
                    self.blocks[pos] = Some(new_node.clone());
                    inserted = true;
                }
            }
        }
        return inserted;
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_bitmap_code() {
        // init 10 elements with 0 (nibble 1= layer, nibble 2 = next index)
        // assume max_level in question is 5 for an element to be inserted
        let byte_index = 10 / 8;
        let bit_offset = 10 % 4;
        let a = [0b0000_0000; 10];
        println!("{:08b}", (a[byte_index] | (1 << (bit_offset + 4)) & 0xF0));
    }
}
