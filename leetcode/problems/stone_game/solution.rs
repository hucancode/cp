use std::cmp::min;
use std::cmp::max;
impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let n = piles.len();
        let all = piles.iter().sum::<i32>();
        let mut alice = vec![vec![0;n];n];// f[i][j] = piles became piles[i,j], turn for alice, maximum stone alice can get?
        let mut bob = vec![vec![0;n];n];// f[i][j] = piles became piles[i,j], turn for bob, maximum stone alice can get?
        for i in 0..n {
            alice[i][i] = piles[i];
            bob[i][i] = 0;
        }
        for len in 1..n {
            for i in 0..n-len {
                let j = i+len;
                // alice play
                let take_head = piles[i] + bob[i+1][j];
                let take_tail = piles[j] + bob[i][j-1];
                alice[i][j] = max(take_head, take_tail);
                // bob play
                let take_head = alice[i+1][j];
                let take_tail = alice[i][j-1];
                bob[i][j] = min(take_head, take_tail);
            }
        }
        let score = alice[0][n-1];
        return score > all - score;
    }
}