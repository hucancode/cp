impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;
        let mut adj = vec![Vec::new();n];
        for e in roads {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let t = e[2];
            adj[u].push((v, t));
            adj[v].push((u, t));
        }
        let mut q = BinaryHeap::new();
        let mut f = vec![0; n];
        let mut cost = vec![i64::MAX; n];
        let mut vis = vec![vec![false; n]; n];
        f[0] = 1;
        cost[0] = 0;
        for &(v, duv) in adj[0].iter() {
            q.push((Reverse(duv as i64), 0, v));
        }
        while let Some((Reverse(du), prev, u)) = q.pop() {
            if vis[prev][u] {
                continue;
            }
            vis[prev][u] = true;
            if cost[u] < du {
                continue;
            }
            cost[u] = du;
            f[u] += f[prev];
            f[u] %= 1000_000_007;
            for &(v, duv) in adj[u].iter() {
                q.push((Reverse(du + duv as i64), u, v));
            }
        }
        //println!("cost {cost:?}, f {f:?}");
        return f[n-1];
    }
}
