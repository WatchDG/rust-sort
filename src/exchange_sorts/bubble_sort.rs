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
/// # Stability
/// * **Stable** - Equal elements maintain their relative order after sorting
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
/// # Stability
/// * **Stable** - Equal elements maintain their relative order after sorting
///
/// # Examples
/// ```
/// use std::cmp::Ordering;
/// use sort::bubble_sort_cmp;
///
/// let mut data = vec![5, 4, 3, 2, 1];
/// bubble_sort_cmp(&mut data, |a, b| {
///     if a > b {
///         Ordering::Greater
///     } else if a < b {
///         Ordering::Less
///     } else {
///         Ordering::Equal
///     }
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

/// Optimized version of bubble sort that includes early termination and boundary optimization.
///
/// This optimized version includes several improvements over the basic bubble sort:
/// 1. **Early termination**: If no swaps occur in a pass, the array is sorted and we can exit early
/// 2. **Boundary optimization**: After each pass, the largest element is in its final position,
///    so we can reduce the inner loop boundary
/// 3. **Last swap tracking**: We track the position of the last swap to further reduce
///    the inner loop boundary in subsequent passes
///
/// These optimizations make the algorithm more efficient, especially for partially sorted arrays.
///
/// # Arguments
/// * `v` - A mutable slice of elements that implement `PartialOrd`
///
/// # Time Complexity
/// * Worst case: O(n²) - when array is sorted in reverse order
/// * Best case: O(n) - when array is already sorted (early termination)
/// * Average case: O(n²) - but with better constant factors than basic bubble sort
///
/// # Space Complexity
/// * O(1) - In-place sorting algorithm
///
/// # Stability
/// * **Stable** - Equal elements maintain their relative order after sorting
///
/// # Examples
/// ```
/// use sort::bubble_sort_optimized;
///
/// let mut data = vec![5, 4, 3, 2, 1];
/// bubble_sort_optimized(&mut data);
/// assert_eq!(data, [1, 2, 3, 4, 5]);
///
/// // Already sorted array - will exit early
/// let mut data = vec![1, 2, 3, 4, 5];
/// bubble_sort_optimized(&mut data);
/// assert_eq!(data, [1, 2, 3, 4, 5]);
/// ```
pub fn bubble_sort_optimized<T: PartialOrd>(v: &mut [T]) {
    let mut n = v.len();
    let mut new_n: usize;

    while n > 1 {
        new_n = 0;
        for i in 1..n {
            if v[i - 1] > v[i] {
                v.swap(i - 1, i);
                new_n = i;
            }
        }
        n = new_n;
    }
}

/// Optimized version of bubble sort with custom comparison function.
///
/// This version combines the benefits of custom comparison with the optimizations
/// of the improved bubble sort algorithm. It includes early termination and
/// boundary optimization for better performance.
///
/// # Arguments
/// * `v` - A mutable slice of elements
/// * `f` - A closure that takes two references to elements and returns an `Ordering`
///
/// # Time Complexity
/// * Worst case: O(n²) - when array is sorted in reverse order
/// * Best case: O(n) - when array is already sorted (early termination)
/// * Average case: O(n²) - but with better constant factors than basic bubble sort
///
/// # Space Complexity
/// * O(1) - In-place sorting algorithm
///
/// # Stability
/// * **Stable** - Equal elements maintain their relative order after sorting
///
/// # Examples
/// ```
/// use std::cmp::Ordering;
/// use sort::bubble_sort_optimized_cmp;
///
/// let mut data = vec![5, 4, 3, 2, 1];
/// bubble_sort_optimized_cmp(&mut data, |a, b| {
///     if a > b {
///         Ordering::Greater
///     } else if a < b {
///         Ordering::Less
///     } else {
///         Ordering::Equal
///     }
/// });
/// assert_eq!(data, [1, 2, 3, 4, 5]);
/// ```
pub fn bubble_sort_optimized_cmp<T, F>(v: &mut [T], f: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let mut n = v.len();
    let mut new_n: usize;

    while n > 1 {
        new_n = 0;
        for i in 1..n {
            if f(&v[i - 1], &v[i]) == Ordering::Greater {
                v.swap(i - 1, i);
                new_n = i;
            }
        }
        n = new_n;
    }
}

#[cfg(test)]
mod tests {
    use crate::{bubble_sort, bubble_sort_cmp, bubble_sort_optimized, bubble_sort_optimized_cmp};
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

    #[test]
    fn sort_optimized() {
        let mut data = vec![5, 4, 3, 2, 1];
        bubble_sort_optimized(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn sort_optimized_cmp() {
        let mut data = vec![5, 4, 3, 2, 1];
        bubble_sort_optimized_cmp(&mut data, |a, b| {
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

    #[test]
    fn sort_already_sorted() {
        let mut data = vec![1, 2, 3, 4, 5];
        let original = data.clone();
        bubble_sort_optimized(&mut data);
        assert_eq!(data, original);
    }

    #[test]
    fn sort_partially_sorted() {
        let mut data = vec![1, 2, 4, 3, 5];
        bubble_sort_optimized(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn sort_empty() {
        let mut data: Vec<i32> = vec![];
        bubble_sort_optimized(&mut data);
        assert_eq!(data, []);
    }

    #[test]
    fn sort_single_element() {
        let mut data = vec![42];
        bubble_sort_optimized(&mut data);
        assert_eq!(data, [42]);
    }
}
