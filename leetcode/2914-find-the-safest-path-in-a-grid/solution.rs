impl Solution {
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        use std::cmp::{min, max};
        use std::collections::VecDeque;
        use std::collections::BinaryHeap;
        let mut n = grid.len();
        let mut distance = vec![vec![n+n-1;n];n];
        let mut vis = vec![vec![false;n];n];
        let mut q = VecDeque::new();
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    q.push_back((i,j, 0));
                }
            }
        }
        while let Some((u,v,d)) = q.pop_front() {
            if vis[u][v] {
                continue;
            }
            vis[u][v] = true;
            distance[u][v] = d;
            if u > 0 {
                q.push_back((u-1,v,d+1));
            }
            if v > 0 {
                q.push_back((u,v-1,d+1));
            }
            if u < n-1 {
                q.push_back((u+1,v,d+1));
            }
            if v < n-1 {
                q.push_back((u,v+1,d+1));
            }
        }
        let mut q = BinaryHeap::new();
        q.push((distance[0][0],0,0));
        let mut vis = vec![vec![false;n];n];
        while let Some((d,u,v)) = q.pop() {
            if u == n-1 && v == n-1 {
                return d as i32;
            }
            if vis[u][v] {
                continue;
            }
            vis[u][v] = true;
            if u > 0 {
                q.push((min(d, distance[u-1][v]), u-1,v));
            }
            if v > 0 {
                q.push((min(d, distance[u][v-1]), u,v-1));
            }
            if u < n-1 {
                q.push((min(d, distance[u+1][v]), u+1,v));
            }
            if v < n-1 {
                q.push((min(d, distance[u][v+1]), u,v+1));
            }
        }
        return 0;
    }
}
