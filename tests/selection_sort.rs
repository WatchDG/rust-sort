use sort::selection_sort;

#[test]
fn sort() {
    let mut data = vec![5, 4, 3, 2, 1];
    selection_sort(&mut data);
    assert_eq!(data, [1, 2, 3, 4, 5]);
}
