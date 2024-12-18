impl Solution {
    pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
        let n = prices.len();
        for i in 0..n {
            if let Some(&y) = prices.iter()
                .skip(i+1)
                .skip_while(|&x| x > &prices[i])
                .next() {
                prices[i] -= y;
            }
        }
        prices
    }
}
