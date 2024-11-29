impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;
        use std::cmp::{max, min};
        if grid[0][1] > 1 && grid[1][0] > 1 {
            return -1;
        }
        let n = grid.len();
        let m = grid[0].len();
        let mut q = BinaryHeap::new();
        let mut f = vec![vec![1000_000;m];n];
        let mut vis = vec![vec![false;m];n];
        f[0][0] = 0;
        q.push((Reverse(0), 0, 0));
        while let Some((Reverse(t), x, y)) = q.pop() {
            if vis[x][y] {
                continue;
            }
            vis[x][y] = true;
            let t = t + 1;
            if x > 0 {
                let x = x - 1;
                let wait = grid[x][y] - t;
                let wait = (max(0,wait)+1)/2*2;
                f[x][y] = min(f[x][y], t+wait);
                q.push((Reverse(f[x][y]), x, y));
            }
            if y > 0 {
                let y = y - 1;
                let wait = grid[x][y] - t;
                let wait = (max(0,wait)+1)/2*2;
                f[x][y] = min(f[x][y], t+wait);
                q.push((Reverse(f[x][y]), x, y));
            }
            if x < n-1 {
                let x = x + 1;
                let wait = grid[x][y] - t;
                let wait = (max(0,wait)+1)/2*2;
                f[x][y] = min(f[x][y], t+wait);
                q.push((Reverse(f[x][y]), x, y));
            }
            if y < m-1 {
                let y = y + 1;
                let wait = grid[x][y] - t;
                let wait = (max(0,wait)+1)/2*2;
                f[x][y] = min(f[x][y], t+wait);
                q.push((Reverse(f[x][y]), x, y));
            }
        }
        f[n-1][m-1]
    }
}
