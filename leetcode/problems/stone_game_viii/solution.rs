use std::cmp::{min, max};
impl Solution {
    pub fn stone_game_viii(stones: Vec<i32>) -> i32 {
        let mut n = stones.len();
        let mut prefix = vec![0;n+1];
        for i in 1..=n {
            prefix[i] = prefix[i-1] + stones[i-1] as i64;
        }
        let mut f = vec![0;n];
        f[n-1] = prefix[n];
        for i in (1..n-1).rev() {
            f[i] = max(f[i+1], prefix[i+1] - f[i+1]);
        }
        return f[1] as i32;

    }
}