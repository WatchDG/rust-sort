use sort::{insertion_sort, insertion_sort_cmp};
use std::cmp::Ordering;

#[test]
fn sort() {
    let mut data = vec![5, 4, 3, 2, 1];
    insertion_sort(&mut data);
    assert_eq!(data, [1, 2, 3, 4, 5]);
}

#[test]
fn sort_cmp() {
    let mut data = vec![5, 4, 3, 2, 1];
    insertion_sort_cmp(&mut data, |a, b| {
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
