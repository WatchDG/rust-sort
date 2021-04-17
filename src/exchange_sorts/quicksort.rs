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

fn get_pivot<T: PartialOrd + Clone>(v: &mut [T]) -> T {
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

    let pivot = get_pivot(v);

    if l < 4 {
        return;
    }

    let mut i = 0;
    let mut j = l - 1;
    loop {
        while i < l && v[i] <= pivot {
            i += 1;
        }
        while v[j] > pivot {
            j -= 1;
        }
        if i >= j {
            break;
        }
        v.swap(i, j);
        i += 1;
        j -= 1;
    }

    let mut m = 0;
    let mut k = i - 1;
    loop {
        while k > 0 && v[k] == pivot {
            k -= 1;
        }
        while v[m] != pivot {
            m += 1;
        }
        if m >= k {
            break;
        }
        v.swap(m, k);
        m += 1;
        k -= 1;
    }

    let (_, rv) = v.split_at_mut(j + 1);
    quicksort(rv);
    let (lv, _) = v.split_at_mut(k + 1);
    quicksort(lv);
}
