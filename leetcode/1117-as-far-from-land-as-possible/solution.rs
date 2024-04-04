impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;
        use std::cmp::{min, max};
        let n = grid.len();
        let mut q = VecDeque::new();
        let mut f = vec![vec![-1;n];n];
        let mut vis = vec![vec![false;n];n];
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    q.push_back((i,j, 0));
                }
            }
        }
        while let Some((i,j,d)) = q.pop_front() {
            if vis[i][j] {
                continue;
            }
            f[i][j] = d;
            vis[i][j] = true;
            if i > 0 {
                q.push_back((i-1,j,d+1));
            }
            if j > 0 {
                q.push_back((i,j-1,d+1));
            }
            if i < n-1 {
                q.push_back((i+1,j,d+1));
            }
            if j < n-1 {
                q.push_back((i,j+1,d+1));
            }
        }
        //println!("{f:?}");
        let mut ret = -1;
        for arr in f {
            for x in arr {
                if x != 0 {
                    ret = max(ret, x);
                }
            }
        }
        ret
    }
}
