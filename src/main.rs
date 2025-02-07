#[cfg(test)] /// this is for testing and experimenting randomly
fn main() {
    let mut x = [0; 10];
    for i in 0..10 {
        x[i] = rand::random::<int32>();
    }
}
fn quick_sort(arr: &mut [i32; 10]) -> &mut [i32; 10] {
    if arr.len() <= 1 {
        return arr;
    }
    // select pivot
    let p = pivot(arr, 0, arr.len() / 2, arr.len() - 1);
    let pivot_value = arr[p];
    arr.swap(p, arr.len() - 1); // move pivot to end
    let partion_index = 0;
    for i in 0..arr.len() - 1{
        if arr[i] < pivot_value{
            arr.swap(i,partition_index);
            partion_index += 1;
        }
    }
    arr.swap(partion_index, arr.len() - 1); /// final position
}
fn pivot(arr: &mut [i32; 10], low: usize, mid: usize, high: usize) -> usize {
    let low_value = arr[low];
    let mid_value = arr[mid];
    let high_value = arr[high];
    if (low_value > mid_value) ^ (low_value > high_value) {
        return low;
    } else if (mid_value > low_value) ^ (mid_value > high_value) {
        return mid;
    } else {
        return high;
    }
}
