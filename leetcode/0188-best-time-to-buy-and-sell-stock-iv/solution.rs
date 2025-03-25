impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        use std::cmp::max;
        let k = k as usize;
        let n = prices.len();
        let mut f = vec![vec![i32::MIN;k+1];n+1];
        let mut g = vec![vec![0;k+1];n+1];
        let mut ret = 0;
        for j in 1..=k {
            for i in 1..=n {
                let price = prices[i-1];
                let keep = max(f[i-1][j], f[i][j-1]);
                let buy = g[i-1][j-1] - price;
                let skip = max(g[i-1][j], g[i][j-1]);
                let sell = f[i-1][j] + price;
                f[i][j] = max(keep, buy);
                g[i][j] = max(skip, sell);
                ret = max(ret, g[i][j]);
            }
        }
        //println!("{f:?}, {g:?}");
        return ret;
    }
}
