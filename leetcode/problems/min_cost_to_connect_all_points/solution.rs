use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let distance = |i: usize, j: usize| {
            let ax = points[i][0];
            let ay = points[i][1];
            let bx = points[j][0];
            let by = points[j][1];
            (ax-bx).abs() + (ay-by).abs()
        };
        let mut q = BinaryHeap::new();
        let mut vis = vec![false;n];
        let mut ret = 0;
        q.push((Reverse(0), 0));
        while let Some((Reverse(cost), u)) = q.pop() {
            if vis[u] {
                continue;
            }
            vis[u] = true;
            ret += cost;
            (0..n).filter(|&v| !vis[v])
                .for_each(|v| q.push((Reverse(distance(u,v)), v)));
        }
        return ret;
    }
}