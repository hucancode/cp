use std::cmp::max;
impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let n = bombs.len();
        let mut ret = 1;
        for i in 0..n {
            let mut vis = vec![false;n];
            let mut q = Vec::new();
            q.push(i);
            while let Some(i) = q.pop() {
                if vis[i] {
                    continue;
                }
                vis[i] = true;
                let xa = bombs[i][0] as f64;
                let ya = bombs[i][1] as f64;
                let ra = bombs[i][2] as f64;
                let candidates = (0..n)
                    .filter(|&j| !vis[j])
                    .filter(|&j| {
                        let xb = bombs[j][0] as f64;
                        let yb = bombs[j][1] as f64;
                        let dx = xa-xb;
                        let dy = ya-yb;
                        dx*dx+dy*dy <= ra*ra
                    });
                q.extend(candidates);
            }
            ret = max(ret, vis.into_iter()
                .filter(|&v| v)
                .count() as i32);
        }
        ret
    }
}