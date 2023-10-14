use std::cmp::{min, max};
impl Solution {
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        let INF = 1000_000_000;
        let n = cost.len();
        let mut f = vec![vec![INF; n+1];n+1];
        // f[i][j] = must build j wall, using upto worker i, what is the minimum cost
        for i in 0..=n {
            f[i][0] = 0;
        }
        for i in 1..=n {
            let cost = cost[i-1];
            let time = time[i-1];
            let wall_built = time + 1;
            // if we use this ith paid worker, we can build 1 wall
            // and allow other free worker to build more time[i] wall
            // => wall_built = time[i] + 1
            //println!("worker {i} cost = {cost}, wall built = {wall_built}");
            for j in 1..=n {
                let need_more = j as i32 - wall_built;
                let pj = max(0, need_more) as usize;
                f[i][j] = min(f[i-1][j], cost + f[i-1][pj]);
            }
            //println!("{f:?}");
        }
        f[n][n]
    }
}