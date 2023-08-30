use std::cmp::{min, max};
impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut prefix = vec![0;n+1];
        for i in 1..=n {
            prefix[i] = prefix[i-1] + stones[i-1];
        }
        // Given nature of the game, Alice will always has bigger score than Bob
        // Alice wants to maximize her score, thus Bob has to maximize his score to keep up
        // So both player would want to maximize their score
        // f[i][j] = maximum different could be archived if we play game with only stones[i,j]
        let mut f = vec![vec![0;n];n];
        for len in 2..=n {
            for i in 0..n-1 {
                let mut j = i+len-1;
                if j >= n {
                    continue;
                }
                let sum_i = prefix[j+1] - prefix[i+1];
                let sum_j = prefix[j] - prefix[i];
                let discard_i = sum_i - f[i+1][j];
                let discard_j = sum_j - f[i][j-1];
                f[i][j] = max(discard_i, discard_j);
                //println!("calculate state {i}~{j}");
                //println!("discard {i} -> {discard_i}, discard {j} -> {discard_j}");
                //println!("f[{i}][{j}] = {}", f[i][j]);
            }
        }
        //println!("{f:?}");
        return f[0][n-1];
    }
}