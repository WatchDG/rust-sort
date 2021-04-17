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

fn heapify<T: PartialOrd>(v: &mut [T]) {
    let l = v.len();
    for i in (0..(l / 2)).rev() {
        let mut j = i;
        loop {
            let mut parent_idx = j;
            let left_child_idx = 2 * j + 1;
            let right_child_idx = 2 * j + 2;
            if left_child_idx < l && v[left_child_idx] < v[parent_idx] {
                parent_idx = left_child_idx;
            }
            if right_child_idx < l && v[right_child_idx] < v[parent_idx] {
                parent_idx = right_child_idx;
            }
            if j == parent_idx {
                break;
            }
            v.swap(j, parent_idx);
            j = parent_idx;
        }
    }
}

pub fn heapsort<T: PartialOrd>(v: &mut [T]) {
    for i in 0..v.len() {
        heapify(&mut v[i..]);
    }
}
