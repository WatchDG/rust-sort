#[cfg(test)]
mod tests {
    use crate::heapsort;

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
}

#[inline]
pub fn heapify<T: PartialOrd>(v: &mut [T], n: usize, mut i: usize) {
    let mut parent_idx = i;
    loop {
        let left_child_idx = 2 * i + 1;
        let right_child_idx = 2 * i + 2;
        if left_child_idx < n && v[left_child_idx] > v[parent_idx] {
            parent_idx = left_child_idx;
        }
        if right_child_idx < n && v[right_child_idx] > v[parent_idx] {
            parent_idx = right_child_idx;
        }
        if i != parent_idx {
            v.swap(i, parent_idx);
            i = parent_idx;
        } else {
            break;
        }
    }
}

#[inline]
pub fn build_heap<T: PartialOrd>(v: &mut [T], n: usize) {
    for i in (0..n / 2).rev() {
        heapify(v, n, i);
    }
}

pub fn heapsort<T: PartialOrd>(v: &mut [T]) {
    let l = v.len();

    build_heap(v, l);

    for i in 1..l {
        v.swap(0, l - i);
        heapify(&mut v[0..l - i], l - i, 0);
    }
}
