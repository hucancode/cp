use std::collections::VecDeque;
use std::cmp::max;
impl Solution {
    pub fn find_max_fish(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;
        let n = grid.len();
        let m = grid[0].len();
        for x in 0..n {
            for y in 0..m {
                if grid[x][y] == 0 {
                    continue;
                }
                let mut score = 0;
                let mut q = Vec::new();
                q.push((x, y));
                while let Some((x, y)) = q.pop() {
                    if x < 0 || x >= n || y < 0 || y >= m {
                        continue;
                    }
                    if grid[x][y] == 0 {
                        continue;
                    }
                    score += grid[x][y];
                    grid[x][y] = 0;
                    q.push((x+1, y));
                    q.push((x, y+1));
                    q.push((x-1, y));
                    q.push((x, y-1));
                }
                ret = max(score, ret);
            }
        }
        return ret;
    }
}