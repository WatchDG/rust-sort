#[cfg(test)]
mod tests {
    use crate::{bubble_sort, bubble_sort_cmp};

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

pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    let l = v.len() - 1;
    for j in 0..l {
        for i in 0..(l - j) {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
            }
        }
    }
}

pub fn bubble_sort_cmp<T, F>(v: &mut [T], f: F)
where
    F: Fn(&T, &T) -> isize,
{
    let l = v.len() - 1;
    for j in 0..l {
        for i in 0..(l - j) {
            if f(&v[i], &v[i + 1]) > 0 {
                v.swap(i, i + 1);
            }
        }
    }
}
