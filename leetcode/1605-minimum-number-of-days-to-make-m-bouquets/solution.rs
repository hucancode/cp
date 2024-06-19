impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        let can_make = |i| {
            let mut bouquets = 0;
            let mut flowers = 0;
            for &day in bloom_day.iter() {
                if day <= i {
                    flowers += 1;
                } else {
                    flowers = 0;
                }
                if flowers >= k {
                    bouquets += 1;
                    flowers = 0;
                }
            }
            return bouquets >= m;
        };
        const INF: i32 = 1000_000_000;
        let mut l = 0;
        let mut r = INF + 1;
        while l < r {
            let m = (l+r)/2;
            if can_make(m) {
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
