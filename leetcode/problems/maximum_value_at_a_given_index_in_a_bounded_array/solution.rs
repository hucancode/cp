impl Solution {
    fn triangle_sum(n: i64, i: i64, v: i64) -> i64 {
        let progress_sum = |i:i64, j:i64| (j+i)*((j-i).abs()+1)/2;
        let partial_progress_sum = |w: i64, h: i64| {
            if w == 0 {
                return 0;
            }
            if h <= w {
                progress_sum(h, 1) + w - h
            } else {
                progress_sum(h, h - w + 1)
            }
        };
        let l = partial_progress_sum(i,v-1);
        let r = partial_progress_sum(n-i-1,v-1);
        l + r + v
    }
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        if n == 1 {
            return max_sum;
        }
        if n == max_sum {
            return 1;
        }
        let n = n as i64;
        let i = index as i64;
        let max_sum = max_sum as i64;
        let mut l = 1;
        let mut r = max_sum;
        while l < r {
            let m = (l+r)/2;
            let x = Self::triangle_sum(n, i, m);
            if x > max_sum {
                r = m;
            } else {
                l = m+1;
            }
        }
        (l - 1) as i32
    }
}