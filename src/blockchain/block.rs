use std::io::Write;

use crate::user::User;
use chrono::prelude::*;
use derive_builder::Builder;
use once_cell::sync::OnceCell;
use sha2::{Digest, Sha256};

#[derive(Builder, Debug, Clone)]
pub struct Block {
    pub id: [u8; 16],
    pub timestamp: i64,
    pub data: Vec<u8>,
    pub hash: usize,
    pub prev_hash: [u8; 16],
    pub signature: [u8; 16],
}

type IdGenerator = fn(User) -> Block;
const GENSIS_BLOCK_GENERATOR: IdGenerator = |user: User| -> Block {
    let mut SHA256 = Sha256::new();
    return BlockBuilder::default()
        .id(user.id)
        .timestamp(chrono::Utc::now().timestamp_millis())
        .data(bincode::serialize(&user).unwrap())
        .hash(SHA256.write(&user.id).unwrap())
        .prev_hash([0; 16])
        .signature([0; 16])
        .build()
        .unwrap();
};
pub struct OneChain {
    pub blocks: Vec<Block>,
    pub created_at: i64,
}

// Simple implementation of a blockchain
impl OneChain {
    pub fn new(user: User, gen: Option<IdGenerator>) -> OneChain {
        let mut blocks = Vec::new();
        let genesis_block = match gen{
            Some(g) => g(user),
            None => GENSIS_BLOCK_GENERATOR(user), // use default unless specified by external callers
        };
        blocks.push(genesis_block);
        OneChain { blocks, created_at: chrono::Utc::now().timestamp_millis() }
    }

    pub fn add_block(&mut self, data: Vec<u8>) {
        let prev_block = self.blocks.last().unwrap();
        todo!()
    }
}
