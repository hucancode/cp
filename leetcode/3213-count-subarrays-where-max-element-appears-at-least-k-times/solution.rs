use std::collections::VecDeque;
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let m = *nums.iter().max().unwrap_or(&1);
        let mut q = VecDeque::new();
        let mut ret = 0;
        let k = k as usize;
        for (i, x) in nums.into_iter().enumerate() {
            if x == m {
                q.push_back(i);
            }
            while q.len() > k {
                q.pop_front();
            }
            if q.len() >= k {
                let j = *q.front().unwrap_or(&0);
                ret += j as i64 + 1;
            }
        }
        ret
    }
}
