use crate::core::block::*;
use sha2::*;

#[repr(align(64))] // align to 64 bytes for cache line alignment
pub struct AlignedPosition {
    pub data: Option<usize>,
    pub padding: [u8; 56], // prevents false sharing
}

pub struct BlockRingBuffer {
    pub cumulative_hash: [u8; 16],
    pub bitmap: [u8; 13],
    pub blocks: [Option<Block>; 100], // pre-allocate fixed length array of blocks
    pub head: AlignedPosition,
    pub tail: AlignedPosition,
    pub size: usize,
    pub capacity: usize,
}
pub trait BlockRingBufferOps {
    fn new() -> BlockRingBuffer;
    fn add(&mut self, phone_number: [u8; 10]) -> bool;
    /// tombstone the block
    fn delete(&mut self, phone_number: [u8; 10]) -> bool;
    /// Reader flushes the read blocks from ring buffer
    fn flush(&mut self) -> bool;
    fn length(&self) -> usize;
}

impl BlockRingBufferOps for BlockRingBuffer {
    fn new() -> Self {
        BlockRingBuffer {
            cumulative_hash: [0; 16],
            bitmap: [0; 13],
            blocks: [None; 100],
            head: AlignedPosition { data: None },
            tail: AlignedPosition { data: None },
            size: 0,
            capacity: 100,
        }
    }

    fn add(&mut self, phone_number: [u8; 10]) -> bool {
        let new_block = Block::new(phone_number, false);
        self._add(phone_number, new_block);
        true
    }

    fn delete(&mut self, phone_number: [u8; 10]) -> bool {
        let new_block = Block::new(phone_number, true);
        self._add(phone_number, new_block);
        true
    }

    fn length(&self) -> usize {
        self.size
    }

    fn flush(&mut self) -> bool {
        if self.size <= 1 || self.head.data.unwrap() == self.tail.data.unwrap() {
            false
        } else {
            let head_index = self.head.data.unwrap();
            let current_head_block = self.blocks[head_index].unwrap();
            // TODO: Flush to memtable
            self.head = AlignedPosition {
                data: Some((self.head.data.unwrap() + 1) % self.capacity),
            };
            true
        }
    }
}

impl BlockRingBuffer {
    /// Internal method to add a new block to the ring buffer.
    /// tail block is updated to point to new block
    fn _add(&mut self, phone_number: [u8; 10], new_block: Block) {
        if self.size == 0 {
            self.head = AlignedPosition { data: Some(0) };
            self.tail = AlignedPosition { data: Some(0) };
            self.blocks[0] = Some(new_block);
            self.size += 1;
            self.bitmap[0] |= 1;
            self.cumulative_hash = sha_hash(&phone_number);
        } else {
            for (a, b) in self.cumulative_hash.iter_mut().zip(sha_hash(&phone_number)) {
                *a ^= b;
            }
            let tail_index = self.tail.data.unwrap();
            // update tail block to point to new block
            let mut current_tail_block = self.blocks[tail_index].unwrap();
            // for new block to be added, current_tail_block -> next
            // needs to be updated to point to new block
            // but it needs to be mod capacity to wrap around
            // e.g. if capacity is 12, tail_index is 11, then new block should be at 0
            let new_tail_index = (tail_index + 1) % self.capacity;
            let new_byte_index = new_tail_index / 8;
            let bit_offset = new_tail_index % 8;
            // reset the bit in bitmap -- replacement scenario
            self.bitmap[new_byte_index] &= !(1 << (bit_offset));
            // set the bit in bitmap
            self.bitmap[new_byte_index] |= 1 << (bit_offset);
            current_tail_block.next = Some(new_tail_index);
            // set tail block back to tail_index
            self.blocks[tail_index] = Some(current_tail_block);
            // new tail block is the new block
            self.blocks[new_tail_index] = Some(new_block);
            self.tail = AlignedPosition { data: Some(new_tail_index) };
            if self.size < self.capacity {
                self.size += 1;
            } else {
                self.size = self.capacity
            }
        }
    }
}
