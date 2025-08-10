use std::cmp::Ordering;

/// Sorts a slice using the bubble sort algorithm.
///
/// Bubble sort is a simple sorting algorithm that repeatedly steps through the list,
/// compares adjacent elements and swaps them if they are in the wrong order.
/// The pass through the list is repeated until no swaps are needed.
///
/// # Arguments
/// * `v` - A mutable slice of elements that implement `PartialOrd`
///
/// # Time Complexity
/// * Worst case: O(n²)
/// * Best case: O(n) when array is already sorted
/// * Average case: O(n²)
///
/// # Space Complexity
/// * O(1) - In-place sorting algorithm
///
/// # Examples
/// ```
/// use sort::bubble_sort;
///
/// let mut data = vec![5, 4, 3, 2, 1];
/// bubble_sort(&mut data);
/// assert_eq!(data, [1, 2, 3, 4, 5]);
/// ```
pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    let l = v.len() - 1;
    for j in 0..l {
        for i in 0..(l - j) {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
            }
        }
    }
}

/// Sorts a slice using the bubble sort algorithm with a custom comparison function.
///
/// This version allows you to specify how elements should be compared using a closure.
/// The comparison function should return `Ordering::Greater` if the first element
/// should come after the second, `Ordering::Less` if it should come before,
/// and `Ordering::Equal` if they are equal.
///
/// # Arguments
/// * `v` - A mutable slice of elements
/// * `f` - A closure that takes two references to elements and returns an `Ordering`
///
/// # Time Complexity
/// * Worst case: O(n²)
/// * Best case: O(n) when array is already sorted
/// * Average case: O(n²)
///
/// # Space Complexity
/// * O(1) - In-place sorting algorithm
///
/// # Examples
/// ```
/// use std::cmp::Ordering;
/// use sort::bubble_sort_cmp;
///
/// let mut data = vec![5, 4, 3, 2, 1];
/// bubble_sort_cmp(&mut data, |a, b| {
///     if a > b { Ordering::Greater }
///     else if a < b { Ordering::Less }
///     else { Ordering::Equal }
/// });
/// assert_eq!(data, [1, 2, 3, 4, 5]);
/// ```
pub fn bubble_sort_cmp<T, F>(v: &mut [T], f: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let l = v.len() - 1;
    for j in 0..l {
        for i in 0..(l - j) {
            if f(&v[i], &v[i + 1]) == Ordering::Greater {
                v.swap(i, i + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{bubble_sort, bubble_sort_cmp};
    use std::cmp::Ordering;

    #[test]
    fn sort() {
        let mut data = vec![5, 4, 3, 2, 1];
        bubble_sort(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn sort_cmp() {
        let mut data = vec![5, 4, 3, 2, 1];
        bubble_sort_cmp(&mut data, |a, b| {
            if a > b {
                Ordering::Greater
            } else if a < b {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        assert_eq!(data, [1, 2, 3, 4, 5]);
    }
}
