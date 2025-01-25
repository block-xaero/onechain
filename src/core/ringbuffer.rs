use chrono::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::LinkedList;

/// Default hasher for the linked list.
pub fn sha_hash(data: [u8; 10]) -> [u8; 16] {
    let mut sha = Sha256::new();
    sha.update(&data);
    let full_hash = sha.finalize();
    return full_hash[..16]
        .try_into()
        .expect("Failed to convert hash to fixed size array");
}
/// Node struct for building hand-rolled linked list.
#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub data: [u8; 16],
    pub timestamp: i64,
    pub next: Option<usize>,
}

impl Block {
    pub fn new(phone_number: [u8; 10]) -> Block {
        let hash: [u8; 16] = sha_hash(phone_number);
        return Block {
            data: hash,
            next: None,
            timestamp: chrono::Utc::now().timestamp_millis(),
        };
    }
}

pub struct BlockRingBuffer {
    pub cumulative_hash: [u8; 16],
    pub bitmap: [u8; 13],
    pub blocks: [Option<Block>; 100], // fixed length array of blocks
    pub head: Option<usize>,
    pub tail: Option<usize>,
    pub size: usize,
    pub capacity: usize,
}
pub trait BlockRingBufferOps {
    fn new() -> BlockRingBuffer;
    fn add(&mut self, phone_number: [u8; 10]);
    fn delete(&mut self) -> Option<[u8; 10]>;
    fn search(&self, phone_number: [u8; 10]) -> Option<&Block>;
    fn length(&self) -> usize;
}

impl BlockRingBufferOps for BlockRingBuffer {
    fn new() -> Self {
        BlockRingBuffer {
            cumulative_hash: [0; 16],
            bitmap: [0; 13],
            blocks: [None; 100],
            head: None,
            tail: None,
            size: 0,
            capacity: 100,
        }
    }

    fn add(&mut self, phone_number: [u8; 10]) {
        let new_block = Block::new(phone_number);
        if (self.size == 0) {
            self.head = Some(0);
            self.tail = Some(0);
            self.blocks[0] = Some(new_block);
            self.size += 1;
            self.cumulative_hash = sha_hash(phone_number);
            return;
        } else {
            self.cumulative_hash = self
                .cumulative_hash
                .iter()
                .into_iter()
                .zip(sha_hash(phone_number))
                .map(|(a, b)| a ^ b)
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap();
            // TODO: Bitmap logic needs revisiting
            let mut tail_index = self.tail.unwrap();
            // update tail block to point to new block
            let mut current_tail_block = self.blocks[tail_index].unwrap();
            // for new block to be added, current_tail_block -> next
            // needs to be updated to point to new block
            // but it needs to be mod capacity to wrap around
            // e.g. if capacity is 12, tail_index is 11, then new block should be at 0
            let new_tail_index = (tail_index + 1) % self.capacity;
            let new_byte_index = new_tail_index / 8;
            let bit_offset = new_tail_index % 8;
            self.bitmap[new_byte_index] |= (1 << (bit_offset));
            current_tail_block.next = Some(new_tail_index);
            // set tail block back to tail_index
            self.blocks[tail_index] = Some(current_tail_block);
            // new tail block is the new block
            self.blocks[tail_index + 1] = Some(new_block);
            self.tail = Some(new_tail_index);
            if self.size < self.capacity {
                self.size += 1;
            } else {
                self.size = self.capacity
            }
        }
    }

    fn delete(&mut self) -> Option<[u8; 10]> {
        todo!()
    }

    fn search(&self, phone_number: [u8; 10]) -> Option<&Block> {
        todo!()
    }

    fn length(&self) -> usize {
        todo!()
    }
}
