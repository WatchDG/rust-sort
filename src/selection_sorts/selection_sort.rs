#[cfg(test)]
mod tests {
    use crate::selection_sort;

    #[test]
    fn sort() {
        let mut data = vec![5, 4, 3, 2, 1];
        selection_sort(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5]);
    }
}

pub fn selection_sort<T: PartialOrd>(v: &mut [T]) {
    let l = v.len();
    for i in 0..(l - 1) {
        let mut idx = i;
        for j in (i + 1)..l {
            if v[idx] > v[j] {
                idx = j;
            }
        }
        if i != idx {
            v.swap(i, idx);
        }
    }
}
