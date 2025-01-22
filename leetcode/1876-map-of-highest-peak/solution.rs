impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = is_water.len();
        let m = is_water[0].len();
        let mut ret = vec![vec![0;m];n];
        let mut vis = vec![vec![false;m];n];
        let mut q = Vec::new();
        for (i, row) in is_water.into_iter().enumerate() {
            for (j, c) in row.into_iter().enumerate() {
                if c == 1 {
                    q.push((i, j));
                }
            } 
        }
        let mut h = 0;
        while !q.is_empty() {
            let mut next = Vec::new();
            while let Some((i,j)) = q.pop() {
                if vis[i][j] {
                    continue;
                }
                vis[i][j] = true;
                ret[i][j] = h;
                if i > 0 {
                    next.push((i-1, j));
                }
                if j > 0 {
                    next.push((i, j-1));
                }
                if i < n-1 {
                    next.push((i+1, j));
                }
                if j < m-1 {
                    next.push((i, j+1));
                }
            }
            h += 1;
            q = next;
        }
        ret
    }
}
