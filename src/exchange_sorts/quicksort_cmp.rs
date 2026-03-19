use std::cmp::Ordering;

fn partition_pivot_cmp<T, F>(vec: &mut [T], pivot: &T, cmp: &F) -> (usize, usize)
where
    F: Fn(&T, &T) -> Ordering,
{
    let length = vec.len();
    let mut left_idx = 0;
    let mut right_idx = length - 1;
    loop {
        while left_idx < length && cmp(&vec[left_idx], pivot) != Ordering::Greater {
            left_idx += 1;
        }
        while cmp(&vec[right_idx], pivot) == Ordering::Greater {
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

fn partition_pivot_block_cmp<T, F>(
    vec: &mut [T],
    pivot: &T,
    mut pivot_idx: usize,
    cmp: &F,
) -> (usize, usize)
where
    F: Fn(&T, &T) -> Ordering,
{
    let mut left_idx = 0;
    pivot_idx -= 1;
    loop {
        while pivot_idx > 0 && cmp(&vec[pivot_idx], pivot) == Ordering::Equal {
            pivot_idx -= 1;
        }
        while cmp(&vec[left_idx], pivot) != Ordering::Equal {
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

fn median_of_three_pivot_cmp<T: Clone, F>(v: &mut [T], cmp: &F) -> T
where
    F: Fn(&T, &T) -> Ordering,
{
    let l = v.len();
    let m = l / 2;
    if cmp(&v[0], &v[m]) == Ordering::Greater {
        v.swap(0, m);
    }
    if cmp(&v[m], &v[l - 1]) == Ordering::Greater {
        v.swap(m, l - 1);
    }
    if cmp(&v[0], &v[m]) == Ordering::Greater {
        v.swap(0, m);
    }
    v[m].clone()
}

/// Sorts a slice using the quicksort algorithm with a custom comparison function.
///
/// This version allows you to specify how elements should be compared using a closure.
/// The comparison function should return `Ordering::Greater` if the first element
/// should come after the second, `Ordering::Less` if it should come before,
/// and `Ordering::Equal` if they are equal.
///
/// This is particularly useful when you need custom sorting logic, such as:
/// - Sorting by a specific field of a struct
/// - Implementing reverse sorting
/// - Sorting with custom business logic
///
/// # Arguments
/// * `v` - A mutable slice of elements
/// * `cmp` - A closure that takes two references to elements and returns an `Ordering`
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
/// use std::cmp::Ordering;
/// use sort::quicksort_cmp;
///
/// // Ascending order
/// let mut data = vec![5, 4, 3, 2, 1];
/// quicksort_cmp(&mut data, &|a, b| a.cmp(b));
/// assert_eq!(data, [1, 2, 3, 4, 5]);
///
/// // Descending order
/// let mut data = vec![1, 2, 3, 4, 5];
/// quicksort_cmp(&mut data, &|a, b| b.cmp(a));
/// assert_eq!(data, [5, 4, 3, 2, 1]);
/// ```
pub fn quicksort_cmp<T: Clone, F>(v: &mut [T], cmp: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let l = v.len();

    if l < 2 {
        return;
    }

    let pivot = median_of_three_pivot_cmp(v, cmp);

    let (pivot_idx, right_idx) = partition_pivot_cmp(v, &pivot, cmp);
    let (left_idx, _) = partition_pivot_block_cmp(v, &pivot, pivot_idx, cmp);
    quicksort_cmp(&mut v[right_idx..], cmp);
    quicksort_cmp(&mut v[..left_idx], cmp);
}
