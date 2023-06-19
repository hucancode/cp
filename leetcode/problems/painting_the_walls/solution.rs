use std::cmp::{min, max};
impl Solution {
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        let INF = cost.iter().sum::<i32>();
        let n = cost.len();
        let mut f = vec![vec![INF; n+1];n+1];
        // f[i][j] = must build j wall, using upto worker i, what is the minimum cost
        for i in 0..=n {
            f[i][0] = 0;
        }
        for i in 1..=n {
            let cost = cost[i-1];
            let time = time[i-1];
            //println!("worker {i} cost = {}, time = {}", cost[i-1], time[i-1]);
            for j in 1..=n {
                // if we use this i paid worker, we can build 1 wall
                // and help other free worker to build more time[i] wall
                // => net gain = time[i] + 1
                let net_gain = time + 1;
                let need_more = j as i32 - net_gain;
                let pj = max(0, need_more) as usize;
                f[i][j] = f[i-1][j];
                f[i][j] = min(f[i][j], cost + f[i-1][pj]);
            }
            //println!("{f:?}");
        }
        f[n][n]
    }
}