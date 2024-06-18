impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, mut worker: Vec<i32>) -> i32 {
        use std::cmp::max;
        worker.sort();
        let n = profit.len();
        let mut q: Vec<(i32, i32)> = difficulty.into_iter()
            .zip(profit.into_iter())
            .collect();
        q.sort_by(|&(a, _), &(b, _)| b.cmp(&a));
        let mut best = 0;
        let mut ret = 0;
        for ability in worker {
            while let Some(&(difficulty, profit)) = q.last() {
                if difficulty > ability {
                    break;
                }
                q.pop();
                best = max(best, profit);
            }
            ret += best;
        }
        return ret;
    }
}
