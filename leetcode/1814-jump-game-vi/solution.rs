impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        use std::cmp::{min, max};
        use std::collections::VecDeque;
        let k = k as usize;
        let n = nums.len();
        let mut f = vec![-1000_000_000;n];
        f[0] = nums[0];
        let mut i = 0;
        while i < n {
            //println!("at {i} f[{i}] = {}", f[i]);
            let mut j = i+1;
            let mut next = j;
            while j < min(n, i+k+1) {
                f[j] = max(f[j], f[i] + nums[j]);
                //println!("from {i} jump to {j} f[{j}] = {}", f[j]);
                if f[j] >= f[i] {
                    next = j;
                    break;
                }
                if f[j] >= f[next] {
                    next = j;
                }
                j += 1;
            }
            i = next;
        }
        f[n-1]
    }
}
