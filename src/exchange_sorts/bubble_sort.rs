#[cfg(test)]
mod test_bubble_sort {
    use crate::bubble_sort;

    #[test]
    fn sort() {
        let mut data = vec![5, 4, 3, 2, 1];
        bubble_sort(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5]);
    }
}

pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    let l = v.len();
    for j in 0..(l - 1) {
        for i in 0..(l - j - 1) {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
            }
        }
    }
}
