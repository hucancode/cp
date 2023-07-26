impl Solution {
    fn safe_ceil(x: f64) -> f64 {
        const EPSILON: f64 = 0.000000001;
        if x - x.floor() < EPSILON { x } else { x.ceil() }
    }
    pub fn min_skips(dist: Vec<i32>, speed: i32, hours_before: i32) -> i32 {
        let n = dist.len();
        let mut f = vec![vec![f64::MAX;n+1];n+1];
        // f[i][j] = cost to finish road i, allowed to skip up to j road
        f[0][0] = 0.0;
        for i in 1..=n {
            let cost = (dist[i-1] as f64)/(speed as f64);
            for j in 0..=i {
                let wait = Self::safe_ceil(f[i-1][j]);
                let skip = if j > 0 { f[i-1][j-1] } else { f64::MAX };
                f[i][j] = if wait < skip { wait } else { skip }  + cost;
            }
        }
        //println!("{:?}", f[n]);
        for j in 0..n {
            if f[n][j] <= hours_before as f64 {
                return j as i32;
            }
        }
        return -1;
    }
}