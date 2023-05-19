use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq)]
enum Color {
    Red,
    Blue,
}

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();
        let mut color: Vec<Option<Color>> = vec![None;n];
        let mut vis = vec![false;n];
        for u in 0..n {
            if vis[u] {
                continue;
            }
            let mut q = VecDeque::new();
            q.push_back((u, Color::Red));
            while let Some((u, c)) = q.pop_front() {
                if let Some(should_be) = color[u] {
                    if c != should_be {
                        return false;
                    }
                }
                color[u] = Some(c);
                if vis[u] {
                    continue;
                }
                vis[u] = true;
                let next = if c == Color::Blue {Color::Red} else {Color::Blue};
                for &v in graph[u].iter() {
                    q.push_back((v as usize, next));
                }
            }
        }
        true
    }
}