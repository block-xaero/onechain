use sha2::{Digest, Sha256};

/// Default hasher for the linked list.
pub fn sha_hash(data: &[u8; 10]) -> [u8; 16] {
    let mut sha = Sha256::new();
    sha.update(&data);
    let full_hash = sha.finalize();
    return full_hash[..16].try_into().expect("Failed to convert hash to fixed size array");
}
#[derive(Debug, Clone, Copy)]
#[repr(align(64))] // align to 64 bytes for cache line alignment
pub struct Block {
    pub data: [u8; 16],
    pub timestamp: i64,
    pub disabled: bool,
    pub next: Option<usize>,
}

impl Block {
    pub fn new(phone_number: [u8; 10], is_disabled: bool) -> Block {
        let hash: [u8; 16] = sha_hash(&phone_number);
        return Block {
            data: hash,
            next: None,
            timestamp: chrono::Utc::now().timestamp_millis(),
            disabled: is_disabled,
        };
    }
}
