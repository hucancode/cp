impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut vis = vec![vec![false;m];n];
        let mut ret = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    vis[i][j] = true;
                }
            }
        }
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 0 && !vis[i][j]{
                    let mut st = Vec::new();
                    st.push((i, j));
                    let mut good = true;
                    while let Some((x, y)) = st.pop() {
                        if x < 0 || x >= n || y < 0 || y >= m {
                            good = false;
                            continue;
                        }
                        if vis[x][y] {
                            continue;
                        }
                        vis[x][y] = true;
                        st.push((x-1,y));
                        st.push((x+1,y));
                        st.push((x,y-1));
                        st.push((x,y+1));
                    }
                    if good {
                        ret += 1;
                    }
                }
            }
        }
        return ret;
    }
}