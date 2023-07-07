use std::cmp::max;
impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let k = k as usize;
        let answer_key: Vec<usize> = answer_key
            .chars()
            .map(|x| if x == 'T' {1} else {0})
            .collect();
        let n = answer_key.len();
        let mut ret = 1;
        let mut l = 0;
        let mut r = 0;
        let mut true_count = answer_key[0];
        loop {
            let distance = r - l + 1;
            let false_count = distance - true_count;
            if true_count <= k || false_count <= k {
                ret = max(ret, distance);
                r += 1;
                if r >= n {
                    break;
                }
                true_count += answer_key[r];
            } else {
                true_count -= answer_key[l];
                l += 1;
            }
        }
        return ret as i32;
    }
}
