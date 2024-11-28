impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;
        let n = grid.len();
        let m = grid[0].len();
        let mut vis = vec![vec![false;m];n];
        let mut q = BinaryHeap::new();
        q.push((Reverse(0), 0, 0));
        while let Some((Reverse(cost), x, y)) = q.pop() {
            if vis[x][y] {
                continue;
            }
            vis[x][y] = true;
            if x == n-1 && y == m-1 {
                return cost;
            }
            if x > 0 {
                q.push((Reverse(cost + grid[x-1][y]), x-1, y));
            }
            if y > 0 {
                q.push((Reverse(cost + grid[x][y-1]), x, y-1));
            }
            if x < n-1 {
                q.push((Reverse(cost + grid[x+1][y]), x+1, y));
            }
            if y < m-1 {
                q.push((Reverse(cost + grid[x][y+1]), x, y+1));
            }
        }
        return 0;
    }
}
