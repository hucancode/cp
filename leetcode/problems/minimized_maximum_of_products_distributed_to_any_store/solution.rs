impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let can_fit = |k| {
            let mut n = n;
            for &q in quantities.iter() {
                n -= q/k + if q%k == 0 {0} else {1};
                if n < 0 {
                    return false;
                }
            }
            return true;
        };
        let mut l = 1;
        let mut r = *quantities.iter().max().unwrap_or(&0);
        while l < r {
            let m = (l+r)/2;
            if !can_fit(m) {
                l = m+1;
            } else {
                r = m;
            }
        }
        return l;
    }
}