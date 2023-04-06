use std::cmp::min;

impl Solution {
    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        let mut batteries = batteries;
        batteries.sort_by(|a, b|b.cmp(a));
        let m = batteries.len();
        let can_last = |x: i64| {
            let mut target = x * n as i64;
            for &m in batteries.iter() {
                target -= min(x, m as i64);
                if target <= 0 {
                    return true;
                }
            }
            return false;
        };
        let mut l = 0i64;
        let mut r = 1+1e14 as i64;
        while l < r {
            let m = (l+r)/2;
            if !can_last(m) {
                r = m;
            } else {
                l = m+1;
            }
        }
        return r-1;
    }
}