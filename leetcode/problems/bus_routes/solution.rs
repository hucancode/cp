use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }
        let n = routes.len();
        let mut routes = routes.into_iter()
            .map(|a| a.into_iter().collect::<HashSet<i32>>())
            .collect::<Vec<HashSet<i32>>>();
        let mut adj = vec![Vec::new();n];
        for i in 0..n {
            for j in i+1..n {
                if routes[i].is_disjoint(&routes[j]) {
                    continue;
                }
                adj[i].push(j);
                adj[j].push(i);
            }
        }
        let mut q = routes.iter().enumerate()
            .filter(|(_, route)| route.contains(&source))
            .map(|(i, _)| (i, 1))
            .collect::<VecDeque<(usize, i32)>>();
        let mut vis = vec![false;n];
        while let Some((u, du)) = q.pop_front() {
            if vis[u] {
                continue;
            }
            vis[u] = true;
            if routes[u].contains(&target) {
                return du;
            }
            for &v in adj[u].iter() {
                q.push_back((v, du+1));
            }
        }
        return -1;
    }
}