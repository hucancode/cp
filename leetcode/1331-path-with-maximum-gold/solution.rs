impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        use std::cmp::max;
        let mut ret = 0;
        let n = grid.len();
        let m = grid[0].len();
        let mut ret = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 0 {
                    continue;
                }
                let mut vis = vec![vec![false;m];n];
                let mut q = vec![(i,j, false)];
                let mut gold = 0;
                while let Some((i,j, backtrack)) = q.pop() {
                    if backtrack {
                        vis[i][j] = false;
                        gold -= grid[i][j];
                        continue;
                    }
                    if vis[i][j] || grid[i][j] == 0 {
                        continue;
                    }
                    vis[i][j] = true;
                    gold += grid[i][j];
                    //println!("visit {i}, {j} gold = {gold}");
                    ret = max(ret, gold);
                    q.push((i, j, true));
                    if i > 0 {
                        q.push((i-1, j, false));
                    }
                    if i < n-1 {
                        q.push((i+1, j, false));
                    }
                    if j > 0 {
                        q.push((i, j-1, false));
                    }
                    if j < m-1 {
                        q.push((i, j+1, false));
                    }
                }
            }
        }
        ret
    }
}
