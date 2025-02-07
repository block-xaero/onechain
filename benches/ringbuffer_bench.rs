use criterion::{criterion_group, criterion_main, Criterion, black_box};
use std::time::Instant;
use onechain::storage::ring_buffer::BlockRingBuffer;
use onechain::core::block::Block;
use onechain::storage::ring_buffer::BlockRingBufferOps;
use proptest::prelude::*;

fn bench_ringbuffer_l1_cache_access(c: &mut Criterion) {
    let mut ring_buffer = BlockRingBuffer::new();

    // Fill the ring buffer with predictable values
    for i in 0..100 {
        ring_buffer.add([(i % 256) as u8; 10]);
    }

    let num_iterations = 1_000_000;

    c.bench_function("ringbuffer L1 cache access", |b| {
        b.iter(|| {
            for i in 0..num_iterations {
                let index = (i % 100) as usize;
                let _block = black_box(&ring_buffer.blocks[index]); // Prevent compiler optimizations
            }
        })
    });
}

fn bench_cache_alignment(c: &mut Criterion) {
    c.bench_function("cache alignment randomized inserts", |b| {
        b.iter(|| {
            let mut ring_buffer = BlockRingBuffer::new();

            let phone_numbers: Vec<[u8; 10]> = (0..100)
                .map(|i| [(i % 256) as u8; 10])
                .collect();

            for phone in phone_numbers.iter() {
                ring_buffer.add(*phone);
            }

            let num_iterations = 1_000_000;
            for i in 0..num_iterations {
                let index = (i % 100) as usize;
                let _block = black_box(&ring_buffer.blocks[index]);
            }
        })
    });
}

// Define the benchmark group
criterion_group!(benches, bench_ringbuffer_l1_cache_access, bench_cache_alignment);
criterion_main!(benches);