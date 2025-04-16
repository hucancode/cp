impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        use std::collections::HashMap;
        let mut count: HashMap<i32, i32> = HashMap::new();
        let mut ret = 0;
        let n = nums.len();
        let mut j = 0;
        let mut score = 0;
        for i in 0..n {
            while score < k && j < n {
                let x = count.entry(nums[j]).or_default();
                score += *x;
                *x += 1;
                j += 1;
            }
            if score >= k {
                ret += (n-j+1) as i64;
            }
            let x = count.entry(nums[i]).or_default();
            *x -= 1;
            score -= *x;
        }
        return ret;
    }
}
