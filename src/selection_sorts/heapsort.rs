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
    let mut sl = (l / 2) as isize - 1;
    while sl >= 0 {
        let mut i = sl as usize;
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
        sl -= 1;
    }
}

pub fn heapsort<T: PartialOrd>(v: &mut [T]) {
    for m in 0..v.len() {
        let (_, rv) = v.split_at_mut(m);
        heapify(rv);
    }
}
