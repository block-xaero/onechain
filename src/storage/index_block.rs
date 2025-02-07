#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexBlock {
    pub index: [u8; 8],
    pub hashed_data: [u8; 16],
    pub offset: usize,
}

impl IndexBlock {
    pub fn new() -> IndexBlock {
        IndexBlock {
            index: [0; 8],
            hashed_data: [0; 16],
            offset: 0,
        }
    }
}
