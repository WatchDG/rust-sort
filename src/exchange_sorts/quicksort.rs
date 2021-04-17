#[cfg(test)]
mod tests {
    use crate::quicksort;

    #[test]
    fn sort_1() {
        let mut data = vec![5, 4, 3, 2, 1];
        quicksort(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn sort_2() {
        let mut data = vec![1, 1];
        quicksort(&mut data);
        assert_eq!(data, [1, 1]);
    }

    #[test]
    fn sort_3() {
        let mut data = vec![1, 2, 2, 2, 1];
        quicksort(&mut data);
        assert_eq!(data, [1, 1, 2, 2, 2]);
    }
}

pub fn _exchange<T: PartialOrd>(vec: &mut [T], pivot: &T) -> (usize, usize) {
    let length = vec.len();
    let mut left_idx = 0;
    let mut right_idx = length - 1;
    loop {
        while left_idx < length && vec[left_idx] <= *pivot {
            left_idx += 1;
        }
        while vec[right_idx] > *pivot {
            right_idx -= 1;
        }
        if left_idx >= right_idx {
            break;
        }
        vec.swap(left_idx, right_idx);
        left_idx += 1;
        right_idx -= 1;
    }
    (left_idx, right_idx + 1)
}

pub fn _move_pivots<T: PartialOrd>(
    vec: &mut [T],
    pivot: &T,
    mut pivot_idx: usize,
) -> (usize, usize) {
    let mut left_idx = 0;
    pivot_idx -= 1;
    loop {
        while pivot_idx > 0 && vec[pivot_idx] == *pivot {
            pivot_idx -= 1;
        }
        while vec[left_idx] != *pivot {
            left_idx += 1;
        }
        if left_idx >= pivot_idx {
            break;
        }
        vec.swap(left_idx, pivot_idx);
        left_idx += 1;
        pivot_idx -= 1;
    }
    (left_idx, pivot_idx)
}

pub fn _pivot<T: PartialOrd + Clone>(v: &mut [T]) -> T {
    let l = v.len();
    let m = l / 2;
    if v[0] > v[m] {
        v.swap(0, m);
    }
    if v[m] > v[l - 1] {
        v.swap(m, l - 1);
    }
    if v[0] > v[m] {
        v.swap(0, m);
    }
    v[m].clone()
}

pub fn quicksort<T: PartialOrd + Clone>(v: &mut [T]) {
    let l = v.len();

    if l < 2 {
        return;
    }

    let pivot = _pivot(v);

    if l < 4 {
        return;
    }

    let (pivot_idx, right_idx) = _exchange(v, &pivot);
    let (left_idx, _) = _move_pivots(v, &pivot, pivot_idx);
    quicksort(&mut v[right_idx..]);
    quicksort(&mut v[..left_idx]);
}
