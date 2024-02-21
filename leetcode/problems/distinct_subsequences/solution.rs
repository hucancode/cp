impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let n = s.len();
        let m = t.len();
        let mut f = vec![vec![0;n+1];m+1];
        for j in 0..=n {
            f[0][j] = 1;
        }
        for i in 1..=m {
            for j in 1..=n {
                if s[j-1] == t[i-1] {
                    f[i][j] += f[i-1][j-1];
                }
                f[i][j] += f[i][j-1];
            }
        }
        //println!("{f:?}");
        f[m][n]
    }
}