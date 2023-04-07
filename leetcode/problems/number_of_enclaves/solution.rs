impl Solution {
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut vis = vec![vec![false;m];n];
        let mut q = Vec::new();
        let mut ret = 0;
        for i in 0..n {
            for j in 0..m {
                q.push((i,j));
                let mut count = 0;
                let mut good = true;
                while let Some((x,y)) = q.pop() {
                    if x < 0 || x >= n || y < 0 || y >= m {
                        good = false;
                        continue;
                    }
                    if vis[x][y] {
                        continue;
                    }
                    vis[x][y] = true;
                    if grid[x][y] == 0 {
                        continue;
                    }
                    count += 1;
                    q.push((x-1, y));
                    q.push((x+1, y));
                    q.push((x, y-1));
                    q.push((x, y+1));
                }
                if good {
                    ret += count;
                }
            }
        }
        return ret;
    }
}