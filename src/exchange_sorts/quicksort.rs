pub(crate) fn partition_pivot<T: PartialOrd>(vec: &mut [T], pivot: &T) -> (usize, usize) {
    let length = vec.len();
    let mut left_idx = 0;
    let mut right_idx = length - 1;
    loop {
        while left_idx < length && vec[left_idx] <= *pivot {
            left_idx += 1;
        }
        while vec[right_idx] > *pivot {
            right_idx -= 1;
        }
        if left_idx >= right_idx {
            break;
        }
        vec.swap(left_idx, right_idx);
        left_idx += 1;
        right_idx -= 1;
    }
    (left_idx, right_idx + 1)
}

pub(crate) fn partition_pivot_block<T: PartialOrd>(
    vec: &mut [T],
    pivot: &T,
    mut pivot_idx: usize,
) -> (usize, usize) {
    let mut left_idx = 0;
    pivot_idx -= 1;
    loop {
        while pivot_idx > 0 && vec[pivot_idx] == *pivot {
            pivot_idx -= 1;
        }
        while vec[left_idx] != *pivot {
            left_idx += 1;
        }
        if left_idx >= pivot_idx {
            break;
        }
        vec.swap(left_idx, pivot_idx);
        left_idx += 1;
        pivot_idx -= 1;
    }
    (left_idx, pivot_idx)
}

pub(crate) fn median_of_three_pivot<T: PartialOrd + Clone>(v: &mut [T]) -> T {
    let l = v.len();
    let m = l / 2;
    if v[0] > v[m] {
        v.swap(0, m);
    }
    if v[m] > v[l - 1] {
        v.swap(m, l - 1);
    }
    if v[0] > v[m] {
        v.swap(0, m);
    }
    v[m].clone()
}

/// Sorts a slice using the quicksort algorithm.
///
/// Quicksort is an efficient, divide-and-conquer sorting algorithm that works by
/// selecting a pivot element and partitioning the array into elements less than and
/// greater than the pivot. The sub-arrays are then sorted recursively.
///
/// This implementation uses median-of-three pivot selection for better performance
/// on partially sorted arrays.
///
/// # Arguments
/// * `v` - A mutable slice of elements that implement `PartialOrd`
///
/// # Time Complexity
/// * Worst case: O(n²) - when array is already sorted or reverse sorted
/// * Average case: O(n log n)
/// * Best case: O(n log n)
///
/// # Space Complexity
/// * O(log n) - due to recursive calls
///
/// # Stability
/// * **Unstable** - equal elements may change relative order
///
/// # Examples
/// ```
/// use sort::quicksort;
///
/// let mut data = vec![5, 4, 3, 2, 1];
/// quicksort(&mut data);
/// assert_eq!(data, [1, 2, 3, 4, 5]);
/// ```
pub fn quicksort<T: PartialOrd + Clone>(v: &mut [T]) {
    let l = v.len();

    if l < 2 {
        return;
    }

    let pivot = median_of_three_pivot(v);

    let (pivot_idx, right_idx) = partition_pivot(v, &pivot);
    let (left_idx, _) = partition_pivot_block(v, &pivot, pivot_idx);
    quicksort(&mut v[right_idx..]);
    quicksort(&mut v[..left_idx]);
}
