impl Solution {
    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        use std::cmp::max;
        let radius_from_center = |adj: &Vec<Vec<usize>>, center: usize| {
            let n = adj.len();
            let mut distance = vec![0i32;n];
            let mut vis = vec![false;n];
            let mut q = vec![center];
            while let Some(u) = q.pop() {
                if vis[u] {
                    continue;
                }
                vis[u] = true;
                for &v in adj[u].iter() {
                    distance[v] = distance[u] + 1;
                    q.push(v);
                }
            }
            distance.into_iter()
                .enumerate()
                .reduce(|(i, di), (j, dj)| if di > dj { (i, di) } else { (j, dj) })
                .unwrap()
        };
        let find_diameter = |adj: &Vec<Vec<usize>>| {
            let (a, _) = radius_from_center(&adj, 0);
            let (b, len) = radius_from_center(&adj, a);
            return len;
        };
        let build_adj_list = |tree: &Vec<Vec<i32>>| {
            let n = tree.len() + 1;
            let mut adj = vec![Vec::new();n];
            for uv in tree.iter() {
                let u = uv[0] as usize;
                let v = uv[1] as usize;
                adj[u].push(v);
                adj[v].push(u);
            }
            adj
        };
        let find_minimax_radius = |adj: &Vec<Vec<usize>>| {
            let n = adj.len();
            if n <= 2 {
                return n as i32 - 1;
            }
            let mut degree: Vec<usize> = adj.iter()
                .map(|a| a.len())
                .collect();
            let mut vis = vec![false;n];
            let mut cost = vec![0;n];
            let mut q = Vec::new();
            for i in 0..n {
                if degree[i] == 1 {
                    q.push(i);
                    cost[i] = 0;
                }
            }
            let mut ret = 0;
            while !q.is_empty() {
                let mut next = Vec::new();
                while let Some(u) = q.pop() {
                    ret = max(ret, cost[u]);
                    if vis[u] {
                        continue;
                    }
                    vis[u] = true;
                    for &v in adj[u].iter() {
                        degree[v] -= 1;
                        cost[v] = max(cost[v], cost[u] + 1);
                        if degree[v] == 1 {
                            next.push(v);
                        }
                    }
                }
                q = next;
            }
            return ret;
        };
        let adj1 = build_adj_list(&edges1);
        let adj2 = build_adj_list(&edges2);
        max(find_minimax_radius(&adj1) + find_minimax_radius(&adj2) + 1, 
            max(find_diameter(&adj1), find_diameter(&adj2)))
    }
}
