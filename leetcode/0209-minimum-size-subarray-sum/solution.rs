impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        use std::cmp::min;
        use std::collections::VecDeque;
        let n = nums.len();
        let mut sum = 0;
        let mut ret = n+1;
        let mut q = VecDeque::new();
        for x in nums {
            sum += x;
            q.push_back(x);
            while sum >= target && !q.is_empty() {
                ret = min(ret, q.len());
                sum -= q.pop_front().unwrap();
            }
        }
        if ret > n { 0 } else { ret as i32 }
    }
}
