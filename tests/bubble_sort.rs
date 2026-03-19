use sort::{bubble_sort, bubble_sort_cmp, bubble_sort_optimized, bubble_sort_optimized_cmp};
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
