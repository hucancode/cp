use std::cmp::min;
use std::cmp::max;
impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        let mut alice = vec![vec![0;n+1];n]; // alice[i][m] = skip i pile, alice is playing what is the maximum stone alice can get
        let mut bob = vec![vec![i32::MAX;n+1];n]; // bob[i][m] = skip i pile, bob is playing what is the maximum stone alice can get
        for i in (0..n).rev() {
            for m in (1..=n).rev() {
                let can_take_all = i+m*2 >= n;
                if can_take_all {
                    bob[i][m] = 0;
                    alice[i][m] = piles.iter().skip(i).sum();
                } else {
                    for x in 1..=m*2 {
                        let next_m = max(m, x);
                        // bob play
                        let take_x = alice[i+x][next_m];
                        bob[i][m] = min(bob[i][m], take_x);
                        // alice play
                        let take_x = piles.iter().skip(i).take(x).sum::<i32>() + 
                            bob[i+x][next_m];
                        alice[i][m] = max(alice[i][m], take_x);
                    }
                }
            }
        }
        return alice[0][1];
    }
}