impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut row = 0;
        let n = query_row as usize;
        let m = query_glass as usize;
        let mut f = vec![vec![0.0;n+1];n+1];
        f[0][0] = poured as f64;
        for i in 0..n {
            for j in 0..=i {
                let delta = f[i][j] - 1.0;
                if delta < f64::EPSILON {
                    continue;
                }
                f[i+1][j] += delta/2.0;
                f[i+1][j+1] += delta/2.0;
            }
        }
        if f[n][m] - 1.0 > f64::EPSILON {
            return 1.0;
        }
        return f[n][m];
    }
}