use std::cmp::max;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut candy = vec![1;n];
        for i in (1..n).filter(|&i| ratings[i] > ratings[i-1]) {
            candy[i] = max(candy[i], candy[i-1] + 1);
        }
        for i in (0..n-1).rev().filter(|&i| ratings[i] > ratings[i+1]) {
            candy[i] = max(candy[i], candy[i+1] + 1);
        }
        candy.iter().sum()
    }
}