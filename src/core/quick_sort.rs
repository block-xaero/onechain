use libc::SCM_RIGHTS;

struct QuickSort;
trait QuickSortOps {
    fn sort(&mut self, ringbuffer_flush_candidate: &mut [u16], pivot: usize);
}

impl QuickSortOps for QuickSort {
    fn sort(&mut self, ringbuffer_flush_candidate: &mut [u16], mut pivot: usize) {
        // move all elements greater than pivot to the right of pivot
        if pivot == usize::MAX {
            pivot = 50;
        }
        for i in 0..100 {
            if ringbuffer_flush_candidate[i] > ringbuffer_flush_candidate[pivot] {
                // move the ith element to the right of pivot
                let tmp = ringbuffer_flush_candidate[i];
                ringbuffer_flush_candidate[i] = ringbuffer_flush_candidate[pivot + 1];
                ringbuffer_flush_candidate[pivot + 1] = tmp;
            }
        }
        self.sort(ringbuffer_flush_candidate[..pivot].as_mut(), pivot / 2);
        self.sort(ringbuffer_flush_candidate[pivot + 1..].as_mut(), (100 - (pivot + 1)) / 2);
    }
}
