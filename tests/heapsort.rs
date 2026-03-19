use sort::heapsort;

#[test]
fn sort_1() {
    let mut data = vec![5, 4, 3, 2, 1];
    heapsort(&mut data);
    assert_eq!(data, [1, 2, 3, 4, 5]);
}

#[test]
fn sort_2() {
    let mut data = vec![8, 7, 6, 5, 4, 6, 6, 3, 2, 1, 0];
    let expected = vec![0, 1, 2, 3, 4, 5, 6, 6, 6, 7, 8];

    heapsort(&mut data);
    assert_eq!(data, expected);
}
