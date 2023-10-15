use std::cmp::min;
impl Solution {
    pub fn num_ways(steps: i32, n: i32) -> i32 {
        let n = n as usize;
        let INF = 1000_000_007;
        let mut f = Vec::new();
        f.push(1);
        for _ in 0..steps {
            let mut g = f.clone();
            g.push(0);
            let n = min(n, f.len());
            for i in 0..n {
                if i > 0 {
                    g[i-1] += f[i];
                    g[i-1] %= INF;
                }
                if i < n {
                    g[i+1] += f[i];
                    g[i+1] %= INF;
                }
            }
            f = g;
        }
        return f[0];
    }
}