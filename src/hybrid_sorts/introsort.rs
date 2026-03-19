use crate::exchange_sorts::quicksort;
use crate::heapsort;
use crate::insertion_sort;

fn _introsort<T: PartialOrd + Clone>(v: &mut [T], depth: usize) {
    let l = v.len();

    if l <= 16 {
        insertion_sort(v);
        return;
    }

    if depth == 0 {
        heapsort(v);
        return;
    }

    let pivot = quicksort::median_of_three_pivot(v);
    let (pivot_idx, right_idx) = quicksort::partition_pivot(v, &pivot);
    let (left_idx, _) = quicksort::partition_pivot_block(v, &pivot, pivot_idx);
    _introsort(&mut v[right_idx..], depth - 1);
    _introsort(&mut v[..left_idx], depth - 1);
}

pub fn introsort<T: PartialOrd + Clone>(v: &mut [T]) {
    let max_depth = ((v.len() as f64).log2() * 2.) as usize;
    _introsort(v, max_depth)
}
