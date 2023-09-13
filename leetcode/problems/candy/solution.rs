use std::cmp::max;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut candy = vec![1;n];
        let it1 = (1..n).map(|i| (i, i-1));
        let it2 = (0..n-1).rev().map(|i| (i, i+1));
        it1.chain(it2)
            .filter(|(i,j)| ratings[*i] > ratings[*j])
            .for_each(|(i,j)| candy[i] = max(candy[i], candy[j] + 1));
        candy.iter().sum()
    }
}