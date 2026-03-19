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
