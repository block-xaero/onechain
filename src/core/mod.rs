// Module: core
pub mod block;
pub mod merkle;
pub mod mpt;
pub mod skip_list;
use crate::storage::ring_buffer::BlockRingBuffer;
use block::Block;

pub fn quick_sort(ringbuffer_flush_candidate: &mut [Option<block::Block>]) {
    if ringbuffer_flush_candidate.len() <= 1 {
        return; // Base case: already sorted
    }
    let last = ringbuffer_flush_candidate.len() - 1;
    let mid = ringbuffer_flush_candidate.len() / 2;
    let pivot_index = pivot_selector(ringbuffer_flush_candidate, 0, mid, last);
    // move pivot to end
    let new_pivot_index = partition(ringbuffer_flush_candidate, pivot_index);
    // sort left and right
    quick_sort(&mut ringbuffer_flush_candidate[..new_pivot_index]);
    quick_sort(&mut ringbuffer_flush_candidate[new_pivot_index + 1..]);
}

fn partition(ringbuffer_flush_candidate: &mut [Option<block::Block>], pivot_index: usize) -> usize {
    // move pivot to end
    let pivot_value = ringbuffer_flush_candidate[pivot_index].unwrap().data;
    ringbuffer_flush_candidate.swap(pivot_index, ringbuffer_flush_candidate.len() - 1);
    let mut j = 0;
    for i in 0..ringbuffer_flush_candidate.len() - 1 {
        if ringbuffer_flush_candidate[i].unwrap().data < pivot_value {
            ringbuffer_flush_candidate.swap(i, j);
            j += 1; // increment partition index
        }
    }
    // move pivot to correct position
    ringbuffer_flush_candidate.swap(j, ringbuffer_flush_candidate.len() - 1);
    return j;
}

/// selects
fn pivot_selector(arr: &mut [Option<block::Block>], a: usize, b: usize, c: usize) -> usize {
    let (val_a, val_b, val_c) = (arr[a].unwrap().data, arr[b].unwrap().data, arr[c].unwrap().data);
    if (val_a > val_b) ^ (val_a > val_c) {
        return a;
    } else if (val_b > val_a) ^ (val_b > val_c) {
        return b;
    } else {
        return c;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /// Helper function to generate a `BlockRingBuffer` with hashed `data` values
    fn create_test_ringbuffer(data_values: &[&[u8; 16]]) -> BlockRingBuffer {
        let mut buffer = BlockRingBuffer::new();
        let mut index = 0;

        for j in 0..100 {
            let data = data_values[index % data_values.len()]; // Repeat in order
            buffer.blocks[j] = Some(Block {
                data: *data, // Copy the provided value
                timestamp: 0,
                disabled: false,
                next: None,
            });

            index += 1; // Move sequentially through the input values
        }

        buffer
    }

    /// Helper function to extract `data` values from the ring buffer after sorting
    fn extract_data(buffer: &BlockRingBuffer) -> Vec<[u8; 16]> {
        buffer.blocks.iter().filter_map(|b| b.map(|block| block.data)).collect()
    }

    /// Test sorting a reverse lexicographically sorted ring buffer
    #[test]
    fn test_quick_sort_reverse_sorted() {
        let mut buffer = create_test_ringbuffer(&[
            b"eeeeeeeeeeeeeeee",
            b"dddddddddddddddd",
            b"cccccccccccccccc",
            b"bbbbbbbbbbbbbbbb",
            b"aaaaaaaaaaaaaaaa",
        ]);

        quick_sort(buffer.blocks.as_mut());

        // Extract and sort expected values manually
        let mut expected = extract_data(&buffer);
        expected.sort();

        assert_eq!(extract_data(&buffer), expected);
    }
}
