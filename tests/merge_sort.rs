use sort::merge_sort;

#[test]
fn sort() {
    let mut data = vec![5, 4, 3, 2, 1];
    let result = merge_sort(&mut data);
    assert_eq!(result, [1, 2, 3, 4, 5]);
}
