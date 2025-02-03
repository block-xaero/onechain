use crate::storage::ring_buffer;

pub mod block;
pub mod merkle;
pub mod mpt;
pub mod skip_list;

pub fn quick_sort(ringbuffer_flush_candidate: &mut [u16]) {
    if ringbuffer_flush_candidate.len() <= 1 {
        return; // Base case: already sorted
    }
    let last = ringbuffer_flush_candidate.len() - 1;
    let mid = ringbuffer_flush_candidate.len() / 2;
    let pivot_index = pivot_selector(ringbuffer_flush_candidate, 0, mid, last);
    // move pivot to end
    let new_pivot_index = qsort(ringbuffer_flush_candidate, pivot_index);
    // sort left and right
    quick_sort(&mut ringbuffer_flush_candidate[..new_pivot_index]);
    quick_sort(&mut ringbuffer_flush_candidate[new_pivot_index + 1..]);
}

fn qsort(ringbuffer_flush_candidate: &mut [u16], pivot_index: usize) -> usize {
    // move pivot to end
    let pivot = ringbuffer_flush_candidate[pivot_index];
    ringbuffer_flush_candidate.swap(pivot_index, ringbuffer_flush_candidate.len() - 1);
    let mut store_index = 0;
    for i in 0..ringbuffer_flush_candidate.len() - 1 {
        if ringbuffer_flush_candidate[i] < pivot {
            ringbuffer_flush_candidate.swap(i, store_index);
            store_index += 1;
        }
    }
}

/// selects
fn pivot_selector(arr: &mut [u16], a: usize, b: usize, c: usize) -> usize {
    let (val_a, val_b, val_c) = (arr[a], arr[b], arr[c]);
    if (val_a > val_b) ^ (val_a > val_c) {
        return a;
    } else if (val_b > val_a) ^ (val_b > val_c) {
        return b;
    } else {
        return c;
    }
}
