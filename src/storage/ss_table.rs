use crate::storage::bloom_filter::BloomFilter;
use crate::storage::data_block::DataBlock;
use crate::storage::footer::Footer;
use crate::storage::index_block::IndexBlock;
use crate::storage::meta_block::MetaBlock;
pub struct SSTableSegment {
    pub bloom_filter: BloomFilter,
    pub index_block: IndexBlock,
    pub data_block: DataBlock,
    pub meta_block: MetaBlock,
    pub footer: Footer,
}

trait SSTableSegmentOps {}
