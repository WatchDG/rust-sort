use std::cmp::Ordering;

/// Sorts a slice using the insertion sort algorithm.
///
/// Insertion sort is a simple sorting algorithm that builds the final sorted array
/// one item at a time. It is much less efficient on large lists than more advanced
/// algorithms such as quicksort, heapsort, or merge sort. However, insertion sort
/// provides several advantages: it is simple to implement, efficient for small data
/// sets, adaptive (i.e., efficient for data sets that are already substantially
/// sorted), stable (i.e., does not change the relative order of elements with equal keys),
/// and in-place (i.e., only requires a constant amount of additional memory space).
///
/// # Arguments
/// * `v` - A mutable slice of elements that implement `PartialOrd`
///
/// # Time Complexity
/// * Worst case: O(n²) - when the array is sorted in reverse order
/// * Best case: O(n) - when the array is already sorted
/// * Average case: O(n²)
///
/// # Space Complexity
/// * O(1) - In-place sorting algorithm
///
/// # Examples
/// ```
/// use sort::insertion_sort;
///
/// let mut data = vec![5, 4, 3, 2, 1];
/// insertion_sort(&mut data);
/// assert_eq!(data, [1, 2, 3, 4, 5]);
/// ```
pub fn insertion_sort<T: PartialOrd>(v: &mut [T]) {
    for i in 1..=v.len() {
        let mut j = i - 1;
        while j > 0 && v[j - 1] > v[j] {
            v.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// Sorts a slice using the insertion sort algorithm with a custom comparison function.
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
/// - Sorting objects that don't implement `PartialOrd`
///
/// # Arguments
/// * `v` - A mutable slice of elements
/// * `f` - A closure that takes two references to elements and returns an `Ordering`
///
/// # Time Complexity
/// * Worst case: O(n²) - when the array is sorted in reverse order
/// * Best case: O(n) - when the array is already sorted
/// * Average case: O(n²)
///
/// # Space Complexity
/// * O(1) - In-place sorting algorithm
///
/// # Examples
/// ```
/// use std::cmp::Ordering;
/// use sort::insertion_sort_cmp;
///
/// let mut data = vec![5, 4, 3, 2, 1];
/// insertion_sort_cmp(&mut data, |a, b| {
///     if a > b {
///         Ordering::Greater
///     } else if a < b {
///         Ordering::Less
///     } else {
///         Ordering::Equal
///     }
/// });
/// assert_eq!(data, [1, 2, 3, 4, 5]);
///
/// // Reverse sorting
/// let mut data = vec![1, 2, 3, 4, 5];
/// insertion_sort_cmp(&mut data, |a, b| b.cmp(a));
/// assert_eq!(data, [5, 4, 3, 2, 1]);
/// ```
pub fn insertion_sort_cmp<T, F>(v: &mut [T], f: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    for i in 1..=v.len() {
        let mut j = i - 1;
        while j > 0 && f(&v[j - 1], &v[j]) == Ordering::Greater {
            v.swap(j - 1, j);
            j -= 1;
        }
    }
}

