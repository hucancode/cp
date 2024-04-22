impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let n = n as usize;
        let mut vis = vec![false;n];
        let mut adj = vec![Vec::new();n];
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            adj[u].push(v);
            adj[v].push(u);
        }
        let mut ret = 0;
        for u in restricted {
            vis[u as usize] = true;
        }
        let mut q = vec![0];
        while let Some(u) = q.pop() {
            if vis[u] {
                continue;
            }
            vis[u] = true;
            ret += 1;
            for &v in adj[u].iter() {
                q.push(v);
            }
        }
        ret
    }
}
