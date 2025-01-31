pub struct IndexBlock {
    pub index: [u8; 8],
    pub hashed_data: [u8; 16],
    pub offset: usize,
}
