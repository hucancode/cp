use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::cmp::min;
impl Solution {
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut vis = vec![vec![false;n];n];
        let mut score = vec![vec![0;n];n];
        let mut q = VecDeque::new();
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    q.push_back((0,i,j));
                }
            }
        }
        while let Some((k,x,y)) = q.pop_front() {
            if vis[x][y] {
                continue;
            }
            vis[x][y] = true;
            score[x][y] = k;
            if x > 0 {
                q.push_back((k+1, x-1, y));
            }
            if x < n-1 {
                q.push_back((k+1, x+1, y));
            }
            if y > 0 {
                q.push_back((k+1, x, y-1));
            }
            if y <n-1 {
                q.push_back((k+1, x, y+1));
            }
        }
        let mut vis = vec![vec![false;n];n];
        let mut q = BinaryHeap::new();
        q.push((1000,0,0));
        while let Some((k,x,y)) = q.pop() {
            if vis[x as usize][y as usize] {
                continue;
            }
            vis[x as usize][y as usize] = true;
            let k = min(k, score[x][y]);
            if(x == n-1 && y == n-1) {
                return k;
            }
            if x > 0 {
                q.push((k, x-1,y));
            }
            if x < n-1 {
                q.push((k, x+1,y));
            }
            if y > 0 {
                q.push((k, x,y-1));
            }
            if y < n-1 {
                q.push((k, x,y+1));
            }
        }
        return 0;
    }
}