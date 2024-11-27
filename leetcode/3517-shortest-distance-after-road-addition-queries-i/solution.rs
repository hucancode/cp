impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::VecDeque;
        let n = n as usize;
        let mut adj = vec![Vec::new();n];
        for i in 0..n-1 {
            adj[i].push(i+1);
        }
        let bfs = |u:usize,adj:&Vec<Vec<usize>>| {
            let mut vis = vec![false;n];
            let mut q = VecDeque::new();
            q.push_back((u, 0));
            while let Some((u, du)) = q.pop_front() {
                if vis[u] {
                    continue;
                }
                vis[u] = true;
                if u == n-1 {
                    return du;
                }
                for &v in adj[u].iter() {
                    q.push_back((v, du+1));
                }
            }
            return 0;
        };
        let mut ret = Vec::new();
        for q in queries {
            let u = q[0] as usize;
            let v = q[1] as usize;
            adj[u].push(v);
            ret.push(bfs(0, &adj));
        }
        ret
    }
}
