impl Solution {
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        if hour <= (dist.len() - 1) as f64 {
            return -1;
        }
        let check = |x| {
            let n = dist.len();
            let mut needed = 0;
            for i in 0..n-1 {
                needed += ((dist[i] as f64)/(x as f64)).ceil() as i32;
                if needed as f64 > hour {
                    break;
                }
            }
            let y = (dist[n-1] as f64)/(x as f64);
            return needed as f64 + y <= hour;
        };
        let mut l = 1;
        let mut r = 10000000;
        while l < r {
            let m = (l+r)/2;
            if check(m as f64) {
                r = m;
            } else {
                l = m+1;
            }
        }
        return l;
    }
}