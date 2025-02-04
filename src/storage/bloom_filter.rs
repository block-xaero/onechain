use twox_hash::XxHash64;

pub const BLOOM_FILTER_SIZE: usize = 150;
pub struct BloomFilter {
    ///    Bloom filter size depends on:
    ///	n = 1000 (number of elements in SSTable segment)
    /// 	p = False positive probability (e.g., 1%)
    /// 	m = Number of bits needed
    /// 	k = Number of hash functions
    /// m = -(n * ln(p)) / (ln(2))²
    /// k = (m / n) * ln(2)
    /// For n = 1000 and p = 1% (0.01):
    /// m ≈ -(1000 * ln(0.01)) / (ln(2))²
    ///   ≈ (1000 * 4.6) / 0.48
    ///   ≈ 9580 bits (≈ 1197 bytes)
    /// k ≈ (9580 / 1000) * ln(2)
    /// ≈ 6.64  (round to **7 hash functions**)
    pub bits: [u8; BLOOM_FILTER_SIZE], // 100 bytes each byte represents
}

pub trait BloomFilterOps {
    fn set_bit(&mut self, index: usize);
    fn check_bit(&self, index: usize) -> bool;
    fn add(&mut self, hashed: &[u8; 16]);
}
impl BloomFilter {
    pub fn new() -> Self {
        BloomFilter { bits: [0; BLOOM_FILTER_SIZE] }
    }

    /// 7 Hash functions used for Bloom Filter:
    /// We use one base hash functions and derive 7 hash functions from it.
    /// 1. xxHash-derived
    /// 2. CityHash-derived
    /// 3. FNV-1a variation
    /// 4. SipHash-derived
    /// 5. WyHash-derived
    /// 6. MetroHash-derived
    /// 7. xxHash-derived
    fn hash(&self, hashed: &[u8; 16], filter_size: usize) -> [usize; 7] {
        let base_hash = XxHash64::oneshot(1234, hashed);
        return [
            (base_hash >> 32) as usize % filter_size,
            (base_hash & 0xFFFFFFFF) as usize % filter_size, // xxHash-derived
            ((base_hash >> 16) & 0xFFFFFFFF) as usize % filter_size, // CityHash-derived
            ((base_hash.wrapping_mul(0x5bd1e995) >> 24) & 0xFFFFFFFF) as usize % filter_size, // FNV-1a variation
            ((base_hash.wrapping_add(0x9e3779b97f4a7c15)) & 0xFFFFFFFF) as usize % filter_size, // SipHash-derived
            ((base_hash.wrapping_mul(0xc6a4a7935bd1e995) >> 17) & 0xFFFFFFFF) as usize
                % filter_size, // WyHash-derived
            ((base_hash.wrapping_add(0xdaba0b6eb09322e3)) & 0xFFFFFFFF) as usize % filter_size, // MetroHash-derived
        ];
    }
}
impl BloomFilterOps for BloomFilter {
    fn set_bit(&mut self, index: usize) {
        let byte_offset = index / 8;
        let bit_offset = index % 8;
        self.bits[byte_offset] |= 1 << bit_offset; // set bit
    }

    fn check_bit(&self, index: usize) -> bool {
        let byte_offset = index / 8;
        let bit_offset = index % 8;
        return self.bits[byte_offset] & (1 << bit_offset) != 0; // check bit
    }

    // Uses 7 hash functions to set bits in the Bloom Filter
    fn add(&mut self, hashed: &[u8; 16]) {
        let hashed_indices = self.hash(hashed, BLOOM_FILTER_SIZE);
        for index in hashed_indices {
            self.set_bit(index);
        }
    }
}
