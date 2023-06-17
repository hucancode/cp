use std::cmp::{max, min};
use std::collections::BinaryHeap;

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let n = dungeon.len();
        let m = dungeon[0].len();
        let mut f = vec![vec![i32::MIN; m]; n]; // f[i][j] = best health when arrived at (i,j)
        let mut ret = i32::MAX;
        let mut q = BinaryHeap::new();
        q.push((dungeon[0][0], dungeon[0][0], 0, 0)); // health needed, current health, i, j
        while let Some((cost, score, i, j)) = q.pop() {
            if f[i][j] >= score {
                continue;
            }
            f[i][j] = score;
            if i == n - 1 && j == m - 1 {
                ret = min(ret, max(0, -cost));
            }
            if i < n - 1 {
                let i = i + 1;
                let score = score + dungeon[i][j];
                let cost = min(cost, score);
                q.push((cost, score, i, j));
            }
            if j < m - 1 {
                let j = j + 1;
                let score = score + dungeon[i][j];
                let cost = min(cost, score);
                q.push((cost, score, i, j));
            }
        }
        return ret + 1;
    }
}
