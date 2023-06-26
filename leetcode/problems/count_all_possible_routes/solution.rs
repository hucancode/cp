use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        const INF: i32 = 1000_000_007;
        let start = start as usize;
        let finish = finish as usize;
        let fuel = fuel as usize;
        let n = locations.len();
        let mut f = vec![vec![0;fuel+1];n];
        let mut vis = vec![vec![false;fuel+1];n];
        f[finish][0] = 1;
        let mut q = BinaryHeap::new();
        q.push((Reverse(0), finish));
        while let Some((Reverse(du),u)) = q.pop() {
            if vis[u][du] {
                continue;
            }
            vis[u][du] = true;
            for v in (0..n).filter(|&v| v != u) {
                let duv = (locations[u] - locations[v]).abs() as usize;
                let dv = du + duv;
                if dv > fuel {
                    continue;
                }
                f[v][dv] += f[u][du];
                f[v][dv] %= INF;
                q.push((Reverse(dv), v));
            }
        }
        f[start].iter().fold(0, |acc, x| (acc + x)%INF)
    }
}