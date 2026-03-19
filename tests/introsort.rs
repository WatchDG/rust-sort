use sort::introsort;

#[test]
fn sort_1() {
    let mut data = vec![5, 4, 3, 2, 1];
    introsort(&mut data);
    assert_eq!(data, [1, 2, 3, 4, 5]);
}

#[test]
fn sort_2() {
    let mut data = vec![1, 1];
    introsort(&mut data);
    assert_eq!(data, [1, 1]);
}

#[test]
fn sort_3() {
    let mut data = vec![1, 2, 2, 2, 1];
    introsort(&mut data);
    assert_eq!(data, [1, 1, 2, 2, 2]);
}

#[test]
fn sort_empty() {
    let mut data: Vec<i32> = vec![];
    introsort(&mut data);
    assert_eq!(data, []);
}

#[test]
fn sort_single_element() {
    let mut data = vec![42];
    introsort(&mut data);
    assert_eq!(data, [42]);
}

#[test]
fn sort_two_elements() {
    let mut data = vec![2, 1];
    introsort(&mut data);
    assert_eq!(data, [1, 2]);

    let mut data = vec![1, 2];
    introsort(&mut data);
    assert_eq!(data, [1, 2]);
}

#[test]
fn sort_three_elements() {
    let mut data = vec![3, 1, 2];
    introsort(&mut data);
    assert_eq!(data, [1, 2, 3]);

    let mut data = vec![2, 3, 1];
    introsort(&mut data);
    assert_eq!(data, [1, 2, 3]);
}

#[test]
fn sort_already_sorted() {
    let mut data = vec![1, 2, 3, 4, 5];
    introsort(&mut data);
    assert_eq!(data, [1, 2, 3, 4, 5]);
}

#[test]
fn sort_reverse_sorted() {
    let mut data = vec![5, 4, 3, 2, 1];
    introsort(&mut data);
    assert_eq!(data, [1, 2, 3, 4, 5]);
}

#[test]
fn sort_all_equal() {
    let mut data = vec![7, 7, 7, 7, 7];
    introsort(&mut data);
    assert_eq!(data, [7, 7, 7, 7, 7]);
}

#[test]
fn sort_many_duplicates() {
    let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    introsort(&mut data);
    assert_eq!(data, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
}

#[test]
fn sort_large_array() {
    let mut data: Vec<i32> = (1..=100).rev().collect();
    introsort(&mut data);
    for i in 1..=100 {
        assert_eq!(data[(i - 1) as usize], i);
    }
}
