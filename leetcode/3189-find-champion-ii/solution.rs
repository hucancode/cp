impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj: Vec<Vec<usize>> = vec![Vec::new();n];
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            adj[u].push(v);
        }
        let is_champion = |u| {
            let mut vis = vec![false;n];
            let mut q: Vec<usize> = vec![u];
            while let Some(u) = q.pop() {
                if vis[u] {
                    continue;
                }
                vis[u] = true;
                q.extend(adj[u].iter());
            }
            return vis.iter().all(|&v| v);
        };
        let mut ret = -1;
        for u in (0..n).filter(|&u| is_champion(u)) {
            if ret == -1 {
                ret = u as i32;
            } else {
                return -1;
            }
        }
        return ret;
    }
}
