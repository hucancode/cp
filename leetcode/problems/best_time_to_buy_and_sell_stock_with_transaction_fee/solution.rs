use std::cmp::max;
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let n = prices.len();
        let mut keep = vec![0;n];
        let mut sell = vec![0;n];
        keep[0] -= prices[0];
        for i in 1..n {
            keep[i] = max(sell[i-1] - prices[i], keep[i-1]);
            sell[i] = max(keep[i-1] + prices[i] - fee, sell[i-1]);
        }
        sell[n-1]
    }
}