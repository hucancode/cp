impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;
        let n = grid.len();
        let m = grid[0].len();
        let mut vis = vec![vec![false;m];n];
        let mut q = BinaryHeap::new();
        q.push((Reverse(0), 0, 0));
        while let Some((Reverse(cost), i, j)) = q.pop() {
            //println!("check {i} {j}");
            if i == n-1 && j == m-1 {
                return cost;
            }
            if vis[i][j] {
                continue;
            }
            vis[i][j] = true;
            let free_dir = grid[i][j];
            if j < m-1 {
                let cost = if free_dir == 1 {cost} else {cost+1};
                q.push((Reverse(cost), i, j+1));
            }
            if j > 0 {
                let cost = if free_dir == 2 {cost} else {cost+1};
                q.push((Reverse(cost), i, j-1));
            }
            if i < n-1 {
                let cost = if free_dir == 3 {cost} else {cost+1};
                q.push((Reverse(cost), i+1, j));
            }
            if i > 0 {
                let cost = if free_dir == 4 {cost} else {cost+1};
                q.push((Reverse(cost), i-1, j));
            }
        }
        return -1;
    }
}
