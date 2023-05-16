use std::cmp::max;
impl Solution {
    pub fn max_moves(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut ret = 0;
        let mut q = Vec::new();
        for i in 0..n {
            q.push((i, 0));
        }
        while let Some((i,j)) = q.pop() {
            if grid[i][j] == 0 {
                continue;
            }
            ret = max(ret, j);
            let x = grid[i][j];
            grid[i][j] = 0;
            let j = j+1;
            if j >= m {
                continue;
            }
            
            for d in [-1,0,1] {
                let i = i as i32 + d;
                if i < 0 || i >= n as i32 || j >= m {
                    continue;
                }
                let i = i as usize;
                if grid[i][j] > x {
                    q.push((i,j));
                }
            }
        }
        ret as i32
    }
}