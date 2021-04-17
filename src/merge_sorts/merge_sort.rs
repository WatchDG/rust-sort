#[cfg(test)]
mod tests {
    use crate::merge_sort;

    #[test]
    fn sort() {
        let mut data = vec![5, 4, 3, 2, 1];
        let result = merge_sort(&mut data);
        assert_eq!(result, [1, 2, 3, 4, 5]);
    }
}

pub fn merge_sort<T: PartialOrd + Clone>(v: &mut [T]) -> Vec<T> {
    let l = v.len();
    if l < 2 {
        return v.to_vec();
    }
    let m = (l as f64 / 2.).floor() as usize;
    let (lp, rp) = v.split_at_mut(m);
    let mut lv = merge_sort(lp);
    let mut rv = merge_sort(rp);
    let lvl = lv.len();
    let rvl = rv.len();
    let mut li = 0;
    let mut ri = 0;
    let mut new_v = Vec::with_capacity(l);
    for _ in 0..l {
        if lv[li] > rv[ri] {
            new_v.push(rv[ri].clone());
            ri += 1;
        } else {
            new_v.push(lv[li].clone());
            li += 1;
        }
        if li == lvl {
            new_v.extend_from_slice(rv.split_off(ri).as_slice());
            break;
        }
        if ri == rvl {
            new_v.extend_from_slice(lv.split_off(li).as_slice());
            break;
        }
    }
    new_v
}
