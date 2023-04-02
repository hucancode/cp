use std::collections::VecDeque;
impl Solution {
    pub fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
        const INF: i32 = 1e9 as i32;
        let n = grid.len();
        let m = grid[0].len();
        let build = || -> Vec<Vec<i32>> {
            let mut ret = vec![vec![INF+20001;m];n];
            let mut q: VecDeque<(usize, usize, i32)> = VecDeque::new();
            for (i, row) in grid.iter().enumerate() {
                for (j, x) in row.iter().enumerate() {
                    if *x == 1 {
                        q.push_back((i,j,0));
                    }
                }
            }
            while let Some((x,y,t)) = q.pop_front() {
                if x >= n || x < 0 || y >= m || y < 0 
                    || grid[x][y] == 2 
                    || ret[x][y] <= t {
                    continue;
                }
                ret[x][y] = t;
                let t = t+1;
                q.push_back((x-1,y,t));
                q.push_back((x+1,y,t));
                q.push_back((x,y-1,t));
                q.push_back((x,y+1,t));
            }
            return ret;
        };
        let fire_time = build();
        //println!("{fire_time:?}");
        let check = |t| -> bool {
            let mut q: VecDeque<(usize, usize, i32)> = VecDeque::new();
            let mut vis = vec![vec![false;m];n];
            q.push_back((0,0,t));
            let mut good = false;
            while let Some((x,y,t)) = q.pop_front() {
                if x >= n || x < 0 || y >= m || y < 0 
                || vis[x][y]
                || grid[x][y] == 2 {
                    continue;
                }
                vis[x][y] = true;
                if x == n-1 && y == m-1 {
                    return fire_time[x][y] >= t;
                }
                if fire_time[x][y] <= t {
                    continue;
                }
                let t = t+1;
                q.push_back((x-1,y,t));
                q.push_back((x+1,y,t));
                q.push_back((x,y-1,t));
                q.push_back((x,y+1,t));
            }
            return false;
        };
        let mut l = 0;
        let mut r = INF+1;
        while l < r {
            let mut m = (l+r)/2;
            let mut good = check(m);
            //println!("check strategy {l}~{r} ({m}), result = {good}");
            if !good {
                r = m;
            } else {
                l = m+1;
            }
        }
        return r-1;
    }
}