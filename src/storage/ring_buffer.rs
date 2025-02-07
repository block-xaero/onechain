use crate::core::{block::*, quick_sort};
use proptest::prelude::*;
use sha2::*;
use std::time::Instant;

use super::mem_table::{MemTable, MemTableOps};

const PADDING: [u8; 56] = [0; 56];
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
    fn add(&mut self, phone_number: [u8; 10]) -> bool;
    /// tombstone the block
    fn delete(&mut self, phone_number: [u8; 10]) -> bool;
    /// Reader flushes the read blocks from ring buffer
    fn flush(&mut self, memtable: &mut MemTable) -> bool;
    fn length(&self) -> usize;
}

impl BlockRingBuffer {
    pub fn new() -> Self {
        BlockRingBuffer {
            cumulative_hash: [0; 16],
            bitmap: [0; 13],
            blocks: [None; 100],
            head: AlignedPosition { data: None, padding: PADDING },
            tail: AlignedPosition { data: None, padding: PADDING },
            size: 0,
            capacity: 100,
        }
    }
}
impl BlockRingBufferOps for BlockRingBuffer {
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

    fn flush(&mut self, mt: &mut MemTable) -> bool {
        if self.size <= 1 || self.head.data.unwrap() == self.tail.data.unwrap() {
            false
        } else {
            if self.size == self.capacity {
                quick_sort(&mut self.blocks);
                let read_index = self.head.data.unwrap();
                // flush to memtable
                for i in read_index..self.size {
                    let block = self.blocks[i].unwrap();
                    mt.add(&block.data, block.disabled);
                }
                return true;
            } else {
                // do not flush until capacity is reached
                return false;
            }
        }
    }
}

impl BlockRingBuffer {
    /// Internal method to add a new block to the ring buffer.
    /// tail block is updated to point to new block
    fn _add(&mut self, phone_number: [u8; 10], new_block: Block) {
        if self.size == 0 {
            self.head = AlignedPosition { data: Some(0), padding: PADDING };
            self.tail = AlignedPosition { data: Some(0), padding: PADDING };
            self.blocks[0] = Some(new_block);
            self.size += 1;
            self.bitmap[0] |= 1;
            self.cumulative_hash = sha_hash(&phone_number);
        } else {
            // vectorized SIMD instruction to update cumulative hash
            let new_cumulative_hash = u128::from_le_bytes(self.cumulative_hash)
                ^ u128::from_le_bytes(sha_hash(&phone_number));
            self.cumulative_hash = new_cumulative_hash.to_le_bytes();
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
            self.tail = AlignedPosition {
                data: Some(new_tail_index),
                padding: PADDING,
            };
            if self.size < self.capacity {
                self.size += 1;
            } else {
                self.size = self.capacity
            }
        }
    }
}

#[test]
fn test_add_and_flush() {
    let mut ring_buffer = BlockRingBuffer::new();

    // Add a few phone numbers
    assert!(ring_buffer.add([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    assert_eq!(ring_buffer.size, 1);
    assert_eq!(ring_buffer.head.data.unwrap(), 0);
    assert_eq!(ring_buffer.tail.data.unwrap(), 0);

    assert!(ring_buffer.add([11, 12, 13, 14, 15, 16, 17, 18, 19, 20]));
    assert_eq!(ring_buffer.size, 2);
    assert_eq!(ring_buffer.tail.data.unwrap(), 1);

    // Flush a block
    assert!(ring_buffer.flush());
    assert_eq!(ring_buffer.size, 2); // Size remains the same, only head moves
    assert_eq!(ring_buffer.head.data.unwrap(), 1);
}

proptest! {
    #[test]
    fn stress_test_ringbuffer_add_delete(ref phone_numbers in prop::collection::vec(prop::array::uniform10(0u8..255), 100)) {
        let mut ring_buffer = BlockRingBuffer::new();

        // Add 100 random phone numbers
        for phone in phone_numbers.iter() {
            assert!(ring_buffer.add(*phone));
        }
        assert_eq!(ring_buffer.size, 100);

        // Random deletions
        for phone in phone_numbers.iter().take(50) {
            assert!(ring_buffer.delete(*phone));
        }
        println!("######## Size: {}", ring_buffer.size);
        assert!(ring_buffer.size <= 100);
    }
}

#[test]
fn test_cache_behavior() {
    let mut ring_buffer = BlockRingBuffer::new();
    let num_iterations = 1_000_000; // High number of iterations to stress L1 cache

    let start = Instant::now();
    for i in 0..num_iterations {
        ring_buffer.add([(i % 256) as u8; 10]);
    }
    let elapsed = start.elapsed();

    println!("Cache-friendly add() took: {:?}", elapsed);
}

#[test]
fn test_overflow() {
    let mut ring_buffer = BlockRingBuffer::new();

    for i in 0..110 {
        ring_buffer.add([(i % 256) as u8; 10]);
    }
    // Ensure size does not exceed 100 (capacity)
    assert_eq!(ring_buffer.size, 100);
}

#[test]
fn test_cumulative_hash() {
    let mut ring_buffer = BlockRingBuffer::new();
    let phone1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let phone2 = [11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

    ring_buffer.add(phone1);
    let hash1 = ring_buffer.cumulative_hash;

    ring_buffer.add(phone2);
    let hash2 = ring_buffer.cumulative_hash;

    assert_ne!(hash1, hash2); // Hash should change after addition
}

#[test]
fn test_bulk_add_performance() {
    let mut ring_buffer = BlockRingBuffer::new();
    let start = Instant::now();

    for i in 0..100_000 {
        ring_buffer.add([(i % 256) as u8; 10]);
    }

    let elapsed = start.elapsed();
    println!("Bulk add (100,000 inserts) took: {:?}", elapsed);
}
