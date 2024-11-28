impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;
        let n = grid.len();
        let m = grid[0].len();
        let mut vis = vec![vec![vec![false;1+k as usize];m];n];
        let mut q = BinaryHeap::new();
        q.push((Reverse(0), k, 0, 0));
        while let Some((Reverse(cost), k, x, y)) = q.pop() {
            if k < 0 {
                continue;
            }
            if vis[x][y][k as usize] {
                continue;
            }
            vis[x][y][k as usize] = true;
            //println!("at {x}-{y} cost {cost}, k {k}");
            if x == n-1 && y == m-1 {
                return cost;
            }
            if x > 0 {
                q.push((Reverse(cost + 1), k - grid[x-1][y], x-1, y));
            }
            if y > 0 {
                q.push((Reverse(cost + 1), k - grid[x][y-1], x, y-1));
            }
            if x < n-1 {
                q.push((Reverse(cost + 1), k - grid[x+1][y], x+1, y));
            }
            if y < m-1 {
                q.push((Reverse(cost + 1), k - grid[x][y+1], x, y+1));
            }
        }
        return -1;
    }
}
