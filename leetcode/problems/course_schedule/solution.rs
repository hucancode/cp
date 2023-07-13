use std::collections::VecDeque;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut needed = vec![Vec::new();n];
        let mut out_degree = vec![0;n];
        for a in prerequisites.iter() {
            let u = a[0] as usize;
            let v = a[1] as usize;
            needed[v].push(u);
            out_degree[u] += 1;
        }
        let mut q: VecDeque<usize> = out_degree.iter()
            .enumerate()
            .filter_map(|(i, &x)| if x == 0 {Some(i)} else {None})
            .collect();
        while let Some(u) = q.pop_front() {
            for &v in needed[u].iter() {
                out_degree[v] -= 1;
                if out_degree[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        out_degree.into_iter().all(|x| x == 0)
    }
}