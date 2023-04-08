impl Solution {
    pub fn preimage_size_fzf(k: i32) -> i32 {
        // return the number of zero at the end of factorial(x)
        let count_zero = |x:i64| {
            let mut k = 5;
            let mut ret = 0;
            while k <= x {
                ret += x/k;
                k *= 5;
            }
            return ret;
        };
        // return the first number with count_zero(x) = k
        let find = |k| {
            let mut l = 0i64;
            let mut r = 1e10 as i64;
            while l < r {
                let m = (l+r)/2;
                if count_zero(m) < k {
                    l = m+1;
                } else {
                    r = m;
                }
            }
            return l;
        };
        let k = k as i64;
        let count = find(k+1) - find(k);
        return count as i32;
    }
}