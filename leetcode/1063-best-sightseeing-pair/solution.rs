impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut best = 0;
        let mut ret = 0;
        for (j, &x) in values.iter().enumerate() {
            let mut score = x - j as i32;
            ret = ret.max(score + best);
            best = best.max(x+j as i32);
        }
        return ret;
    }
}
