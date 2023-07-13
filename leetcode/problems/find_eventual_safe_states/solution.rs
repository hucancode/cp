use std::collections::VecDeque;
impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut incomming = vec![Vec::new();n];
        let mut out_degree = vec![0;n];
        for (u, adj) in graph.iter().enumerate() {
            for &v in adj.iter() {
                incomming[v as usize].push(u);
                out_degree[u] += 1;
            }
        }
        let mut q: VecDeque<usize> = out_degree.iter()
            .enumerate()
            .filter_map(|(i, &x)| if x == 0 {Some(i)} else {None})
            .collect();
        while let Some(u) = q.pop_front() {
            for &v in incomming[u].iter() {
                out_degree[v] -= 1;
                if out_degree[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        out_degree.into_iter()
            .enumerate()
            .filter_map(|(i, x)| if x == 0 {Some(i as i32)} else {None})
            .collect()
    }
}