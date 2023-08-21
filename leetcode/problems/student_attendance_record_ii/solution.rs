impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let INF = 1_000_000_007;
        let n = n as usize;
        let mut f = vec![vec![vec![0i64;2];3];n]; // f[i][j][k] = at day i, late j day consecutively, k absent, how many possibility?
        f[0][0][0] = 1;
        f[0][0][1] = 1;
        f[0][1][0] = 1;
        for i in 1..n {
            let present = f[i-1][0][0] + f[i-1][1][0] + f[i-1][2][0]; // keep 0 absent, can present at any time regardless of late record
            f[i][0][0] = present % INF;
            let present = f[i-1][0][1] + f[i-1][1][1] + f[i-1][2][1]; // keep 1 absent, can only present if previously absent
            let absent = f[i-1][0][0] + f[i-1][1][0] + f[i-1][2][0]; // keep 1 absent, can only absent if never absent before 
            f[i][0][1] = (present + absent) % INF;
            for j in 1..3 {
                f[i][j][0] = f[i-1][j-1][0]; // the only choice is go late
                f[i][j][1] = f[i-1][j-1][1]; // the only choice is go late
            }
        }
        let mut ret = 0;
        for late in 0..3 {
            for absent in 0..2 {
                ret += f[n-1][late][absent];
                ret %= INF;
            }
        }
        ret as i32
    }
}