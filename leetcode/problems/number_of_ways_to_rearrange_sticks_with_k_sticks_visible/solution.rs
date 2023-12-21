impl Solution {
    pub fn rearrange_sticks(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let INF = 1000_000_007;
        let mut f = vec![vec![0i64; k+1]; n+1];
        // f[i][j] = how many ways to use i stick, and need to make j stick visible
        f[0][0] = 1;
        for j in 1..=k {
            for i in j..=n {
                // put first
                f[i][j] += f[i-1][j-1];
                // put back
                f[i][j] += f[i-1][j]*(i as i64 - 1);
                f[i][j] %= INF;
            }
        }
        return f[n][k] as i32;
    }
}