#[cfg(test)]
mod tests {
    use crate::heapsort;

    #[test]
    fn sort() {
        let mut data = vec![5, 4, 3, 2, 1];
        heapsort(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5]);
    }
}

fn _heapify<T: PartialOrd>(v: &mut [T], mut i: usize) {
    let l = v.len();
    loop {
        let mut parent_idx = i;
        let left_child_idx = 2 * i + 1;
        let right_child_idx = 2 * i + 2;
        if left_child_idx < l && v[left_child_idx] < v[parent_idx] {
            parent_idx = left_child_idx;
        }
        if right_child_idx < l && v[right_child_idx] < v[parent_idx] {
            parent_idx = right_child_idx;
        }
        if i == parent_idx {
            break;
        }
        v.swap(i, parent_idx);
        i = parent_idx;
    }
}

pub fn heapsort<T: PartialOrd>(v: &mut [T]) {
    let l = v.len();
    for i in (0..(l / 2)).rev() {
        _heapify(v, i);
    }

    for i in 0..l {
        _heapify(&mut v[i..], 0);
    }
}
