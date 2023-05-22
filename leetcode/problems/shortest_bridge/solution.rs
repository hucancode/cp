use std::cmp::min;
use std::collections::VecDeque;
impl Solution {
    pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
        const INF: i32 = 10000;
        let n = grid.len();
        let m = grid[0].len();
        let mut vis = vec![vec![false; m];n];
        let mut dis = vec![vec![INF; m];n];
        let mut ret = INF;
        let mut q = VecDeque::new();
        let mut q2 = VecDeque::new();
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    q.push_back((i, j));
                    break;
                }
            }
            if !q.is_empty() {
                break;
            }
        }
        while let Some((i,j)) = q.pop_front() {
            if vis[i][j] {
                continue;
            }
            vis[i][j] = true;
            if grid[i][j] == 1 {
                dis[i][j] = 0;
                if i < n-1 {
                    q.push_back((i+1, j));
                }
                if i > 0 {
                    q.push_back((i-1, j));
                }
                if j < m-1 {
                    q.push_back((i, j+1));
                }
                if j > 0 {
                    q.push_back((i, j-1));
                }
            } else {
                q2.push_back((i,j,0));
            }
        }
        while let Some((i,j,d)) = q2.pop_front() {
            if dis[i][j] <= d {
                continue;
            }
            dis[i][j] = d;
            //println!("dis[{i}][{j}] = {d}");
            if grid[i][j] == 1 {
                ret = min(ret, d);
            } else {
                if i < n-1 {
                    q2.push_back((i+1, j, d+1));
                }
                if i > 0 {
                    q2.push_back((i-1, j, d+1));
                }
                if j < m-1 {
                    q2.push_back((i, j+1, d+1));
                }
                if j > 0 {
                    q2.push_back((i, j-1, d+1));
                }
            }
        }
        ret
    }
}