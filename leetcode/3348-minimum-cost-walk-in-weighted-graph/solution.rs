impl Solution {
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::{HashMap, HashSet};
        let n = n as usize;
        let mut components: HashMap<usize, usize> = HashMap::new();
        let mut vis: HashSet<(usize, usize, i32)> = HashSet::new();
        let mut costs: HashMap<usize, i32> = HashMap::new();
        let mut adj = vec![Vec::new();n];
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let w = e[2];
            adj[u].push((v, w));
            adj[v].push((u, w));
        }
        let mut dfs = |u: usize, component: usize| {
            if components.get(&u).is_some() {
                return;
            }
            components.insert(u, component);
            costs.insert(u, !0);
            let mut q: Vec<(usize, usize, i32)> = Vec::new();
            for &(v, w) in adj[u].iter() {
                q.push((u, v, w));
            }
            while let Some((u, v, w)) = q.pop() {
                if !vis.insert((u, v, w)) {
                    continue;
                }
                vis.insert((v, u, w));
                components.insert(v, component);
                costs.entry(component)
                    .and_modify(|c| *c &= w)
                    .or_insert(w);
                //println!("cost {component} &= {w} = {}", costs[&component]);
                for &(next, w) in adj[v].iter() {
                    q.push((v, next, w));
                }
            }
        };
        for u in 0..n {
            dfs(u, u);
        }
        //println!("{components:?}, {costs:?}");
        query.into_iter()
            .map(|a| (a[0] as usize, a[1] as usize))
            .map(|(s, t)| {
                if components[&s] != components[&t] {
                    -1
                } else {
                    costs[&components[&s]] as i32
                }
            })
            .collect()
    }
}
