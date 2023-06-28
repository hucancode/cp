impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        const INF: i32 = 1000_000_007;
        let n = n as usize;
        let k = k as usize;
        let target = target as usize;
        let mut f = vec![vec![0;1+target];1+n];
        f[0][0] = 1;
        for i in 0..n {
            for j in 0..=target {
                if f[i][j] == 0 {
                    continue;
                }
                for x in 1..=k {
                    let next = j + x;
                    if next > target {
                        break;
                    }
                    f[i+1][next] += f[i][j];
                    f[i+1][next] %= INF;
                }
            }
        }
        return f[n][target];
    }
}