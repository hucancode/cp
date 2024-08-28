impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let n = grid1.len();
        let m = grid1[0].len();
        let mut ret = 0;
        let mut vis = vec![vec![false;m];n];
        let mut dfs = |i: usize, j: usize| {
            if vis[i][j] || grid2[i][j] == 0 {
                return false;
            }
            let mut q = Vec::new();
            q.push((i,j));
            let mut ret = true;
            while let Some((i,j)) = q.pop() {
                if vis[i][j] {
                    continue;
                }
                vis[i][j] = true;
                if grid2[i][j] == 0 {
                    continue;
                }
                if grid1[i][j] == 0 {
                    ret = false;
                }
                if i < n-1 {
                    q.push((i+1,j));
                }
                if i > 0 {
                    q.push((i-1,j));
                }
                if j < m-1 {
                    q.push((i,j+1));
                }
                if j > 0 {
                    q.push((i,j-1));
                }
            }
            //println!("dfs {i} {j} -> {ret}");
            return ret;
        };
        for i in 0..n {
            for j in 0..m {
                if dfs(i,j) {
                    ret +=1;
                }
            }
        }
        return ret;
    }
}
