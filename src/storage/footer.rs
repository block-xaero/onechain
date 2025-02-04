pub struct Footer {
    pub magic_number: u32,
    pub checksum: u32,
    pub max_key: [u8; 16],
    pub min_key: [u8; 16],
}
