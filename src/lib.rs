/// https://en.wikipedia.org/wiki/Quickselect
pub fn quick_select(xs: &mut [i32], k: usize) -> i32 {
    if k == 1 && xs.len() == 1 {
        return xs[0];
    }
    let n = xs.len();
    let pivot = xs[n - 1];
    let mut i = 0;
    for j in 0..(n - 1) {
        if xs[j] <= pivot {
            xs.swap(i, j);
            i += 1;
        }
    }
    xs.swap(i, n - 1);
    if i + 1 == k {
        return pivot;
    } else if i + 1 < k {
        return quick_select(&mut xs[(i + 1)..], k - (i + 1));
    } else {
        return quick_select(&mut xs[0..i], k);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(quick_select(&mut vec![1, 2, 3, 4], 3), 3);
        assert_eq!(quick_select(&mut vec![1, 4, 2, 3, 5], 3), 3);
    }
}
