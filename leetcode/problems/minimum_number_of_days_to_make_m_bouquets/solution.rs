impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        const INF:i32 = 1000_000_000;
        let n = bloom_day.len();
        let check = |day| {
            let mut needed = m;
            let mut streak = 0;
            for i in 0..n {
                if bloom_day[i] <= day {
                    streak += 1;
                } else {
                    needed -= streak/k;
                    streak = 0;
                }
            }
            needed -= streak/k;
            return needed <= 0;
        };
        let mut l = 0;
        let mut r = INF + 1;
        while l < r {
            let m = (l+r)/2;
            if check(m) {
                r = m;
            } else {
                l = m+1;
            }
        }
        if l > INF {
            return -1;
        }
        return l;
    }
}