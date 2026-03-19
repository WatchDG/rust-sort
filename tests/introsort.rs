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
