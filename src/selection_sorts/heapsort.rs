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

fn _heapify<T: PartialOrd>(v: &mut [T], start: usize, end: usize) {
    let mut root = start;
    loop {
        let mut child = root * 2 + 1;

        if child > end {
            break;
        };

        if child + 1 <= end && v[child] < v[child + 1] {
            child += 1;
        }

        if v[root] < v[child] {
            v.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}

pub fn heapsort<T: PartialOrd>(v: &mut [T]) {
    let length = v.len();
    for start in (0..((length - 2) / 2) + 1).rev() {
        _heapify(v, start, length - 1);
    }

    for end in (1..length).rev() {
        v.swap(end, 0);
        _heapify(v, 0, end - 1);
    }
}
