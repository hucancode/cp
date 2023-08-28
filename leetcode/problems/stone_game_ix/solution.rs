use std::collections::HashMap;
impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        let n = stones.len();
        let mut group = vec![0i32;3];
        for x in stones {
            group[(x%3) as usize] += 1;
        }
        if group[0]%2 == 0  {
            group[1] > 0 && group[2] > 0
        } else {
            (group[1] - group[2]).abs() > 2
        }
    }
}