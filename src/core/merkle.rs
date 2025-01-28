use crate::core::block::Block;
pub struct MerkleTree {
    pub root: Block,
    pub nodes: Vec<Block>,
    pub size: usize,
    pub capacity: usize,
}