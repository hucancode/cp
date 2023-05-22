use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let mut neighbors: HashMap<String, Vec<(String, f64)>> = HashMap::new();
        for (l,r)  in equations.into_iter().zip(values.into_iter()) {
            let a = &l[0];
            let b = &l[1];
            let c = r;
            let cr = 1.0/r;
            neighbors.entry(a.to_string())
                .or_default()
                .push((b.to_string(), c));
            neighbors.entry(b.to_string())
                .or_default()
                .push((a.to_string(), cr));
        }
        queries.iter()
            .map(|l| {
                let a = &l[0];
                let b = &l[1];
                let mut vis = HashSet::new();
                let mut q = VecDeque::new();
                if (neighbors.contains_key(a)) {
                    q.push_back((a, 1.0));
                }
                while let Some((u, value)) = q.pop_front() {
                    if vis.contains(u) {
                        continue;
                    }
                    vis.insert(u);
                    if u == b {
                        return value;
                    }
                    if neighbors.contains_key(u) {
                        for (v, w) in neighbors[u].iter() {
                            q.push_back((v, w*value));
                        }
                    }
                }
                -1.0
            })
            .collect()
    }
}