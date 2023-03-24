use std::collections::VecDeque;
impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut go_in: Vec<Vec<usize>> = vec![vec![];n as usize];
        let mut go_out: Vec<Vec<usize>> = vec![vec![];n as usize];
        let mut vis: Vec<bool> = vec![false; n as usize];
        for edge in connections.iter() {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            go_out[a].push(b);
            go_in[b].push(a);
        }
        let mut ret = 0;
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(0);
        while let Some(u) = q.pop_front() {
            if vis[u] {
                continue;
            }
            vis[u] = true;
            go_in[u]
                .iter()
                .filter(|&&v| !vis[v])
                .for_each(|&v| {
                    q.push_back(v);
                });
            go_out[u]
                .iter()
                .filter(|&&v| !vis[v])
                .for_each(|&v| {
                    q.push_back(v);
                    ret += 1;
                });
        }
        return ret;
    }
}