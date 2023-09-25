impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let INF = 1000_000_007;
        let n = n as usize;
        let p0 = min_profit as usize;
        let p = profit.iter().sum::<i32>() as usize;
        let c = group.len();
        let mut f = vec![vec![vec![0;c+1];p+1]; n+1];
        // f(n, p, c), how many ways to use team exact n men, to generate exact p profit, using first c crimes
        f[0][0][0] = 1;
        for i in 0..=n {
            for j in 0..=p {
                for k in 0..c {
                    if(f[i][j][k] == 0) {
                        continue;
                    }
                    f[i][j][k+1] += f[i][j][k];
                    f[i][j][k+1] %= INF;
                    let men = i+group[k] as usize;
                    if men > n {
                        continue;
                    }
                    let gain = j+profit[k] as usize;
                    f[men][gain][k+1] += f[i][j][k];
                    f[men][gain][k+1] %= INF;
                }
            }
        }
        let mut ret = 0;
        for i in 0..=n {
            for j in p0..=p {
                ret += f[i][j][c];
                ret %= INF;
            }
        }
        //println!("{f:?}");
        return ret;
    }
}