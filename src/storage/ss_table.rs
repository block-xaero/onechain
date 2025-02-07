use std::io::Error;

use serde::Serialize;

use crate::core::skip_list::SkipList;
use crate::storage::bloom_filter::BloomFilter;
use crate::storage::data_block::DataBlock;
use crate::storage::footer::Footer;
use crate::storage::index_block::IndexBlock;
use crate::storage::meta_block::MetaBlock;
use crate::sys::{get_page_size, mmap_opt};

use super::bloom_filter::BloomFilterOps;
use super::data_block;
use super::mem_table::MemTable;

///
/// SSTableSegment is a segment of SSTable file.
///
///
/// +----------------------------------------------------+
// |                   SSTableSegment                   |
// +----------------------+----------------------------+
// | Bloom Filter        | Index Block                  |
// | (For fast lookup)   | (Key → Offset mapping)       |
// +----------------------+----------------------------+
// |                     Data Block                     |
// |  (Sorted key-value pairs, stored in sorted order)  |
// +----------------------------------------------------+
// |                  Metadata Block                    |
// | (Compression, timestamps, merge info, etc.)        |
// +----------------------------------------------------+
// |                     Footer                         |
// | (Magic number, version, checksum, etc.)            |
// +----------------------------------------------------+

// Data Block values:
// ```ascii
// +-----------+-----------+--------------------+
// | Key       | Offset    | Value              |
// +-----------+-----------+--------------------+
// | "apple"   | 0x2000    | "fruit"            |
// | "banana"  | 0x2010    | "yellow fruit"     |
// | "cherry"  | 0x2020    | "red fruit"        |
// +-----------+-----------+--------------------+
// ```
// 	•	Keys are sorted lexicographically (e.g., "apple" < "banana" < "cherry").
// 	•	The Index Block maps keys to Data Block offsets.
// 	•	The Bloom Filter helps avoid unnecessary lookups.

pub struct SSTableSegment {
    pub bloom_filter: BloomFilter,
    pub index_block: IndexBlock,
    pub data_block: DataBlock,
    pub meta_block: MetaBlock,
    pub footer: Footer,
}

trait SSTableSegmentOps {
    fn create_segment(&mut self) -> std::result::Result<&mut SSTableSegment, Error>;
}

impl SSTableSegmentOps for SSTableSegment {
    fn create(&mut self, data_block: SkipList) -> std::result::Result<&mut SSTableSegment, Error> {
        let segment_path = format!("sstable-{}.segment", chrono::Utc::now().timestamp_millis());
        let mut mmap_buffer  = mmap_opt(&segment_path)?;
        let offset = 0;
        let page_size = get_page_size();
        // fill 16 KB page with bloom filter, index block, meta block and footer
        while offset < page_size {
            // bloom filter
            let bf = BloomFilter::new();
            let blocks = data_block.blocks;
            for i  in 0..blocks.len() {
                let b = blocks[i];
                if b.is_some() {
                    let block = b.unwrap();
                    bf.add(&block.data);
                }
            }
            let size = size_of_val(&bf);
            mmap_buffer[offset..offset+size].copy_from_slice(&bf);
            // index block
            let ib = IndexBlock::new();
            offset +=size;
        }
        
        Ok(self)
    }
}
