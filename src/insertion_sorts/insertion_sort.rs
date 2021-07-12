#[cfg(test)]
mod tests {
    use crate::{insertion_sort, insertion_sort_cmp};

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
                1
            } else if a < b {
                -1
            } else {
                0
            }
        });
        assert_eq!(data, [1, 2, 3, 4, 5]);
    }
}

pub fn insertion_sort<T: PartialOrd>(v: &mut [T]) {
    for i in 1..=v.len() {
        let mut j = i - 1;
        while j > 0 && v[j - 1] > v[j] {
            v.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn insertion_sort_cmp<T, F>(v: &mut [T], f: F)
where
    F: Fn(&T, &T) -> isize,
{
    for i in 1..=v.len() {
        let mut j = i - 1;
        while j > 0 && f(&v[j - 1], &v[j]) > 0 {
            v.swap(j - 1, j);
            j -= 1;
        }
    }
}
