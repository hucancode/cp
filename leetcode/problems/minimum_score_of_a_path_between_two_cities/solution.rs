use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp::min;
impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut vis = vec![false; n as usize];
        let mut adj: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
        for r in &roads {
            let mut a = r[0] - 1;
            let mut b = r[1] - 1;
            let distance = r[2];
            adj.entry(a).or_default().push((b,distance));
            adj.entry(b).or_default().push((a,distance));
        }
        let mut q = VecDeque::new();
        q.push_back(0);
        let mut ret = 1e4 as i32;
        while let Some(u) = q.pop_front() {
            if(vis[u as usize]) {
                continue;
            }
            vis[u as usize] = true;
            for (v, d) in adj.entry(u).or_default() {
                q.push_back(*v);
                ret = min(ret, *d);
            }
        }
        return ret;
    }
}