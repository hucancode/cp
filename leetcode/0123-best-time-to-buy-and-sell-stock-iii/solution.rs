impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        use std::cmp::max;
        let maxk = 2;
        let mut ret = 0;
        let n = prices.len();
        if n < 2 {
            return 0;
        }
        let mut f = vec![vec![0;n];maxk+1];
        let mut g = vec![vec![0;n];maxk+1];
        // f[i][j] = max profit if buy at j, have bought i times
        // g[i][j] = max profit if sell at j, have sold i times
        for i in 1..=maxk {
            for j in 0..i {
                g[i][j] = -1000_000;
                f[i][j] = if j>0 {f[i][j-1]} else {0} - prices[j];
            }
        }
        for i in 1..=maxk {
            for j in 1..n {
                let buy = g[i-1][j-1] - prices[j];
                let skip = f[i][j-1];
                f[i][j] = max(buy, skip);
            }
            for j in 1..n {
                let sell = f[i][j-1]+prices[j];
                let skip = g[i][j-1];
                g[i][j] = max(sell, skip);
                ret = max(ret, g[i][j]);
            }
        }
        return ret;
    }
}
