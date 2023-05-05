use std::collections::VecDeque;
impl Solution {
    pub fn get_subarray_beauty(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut count = vec![0;51];
        let mut q = VecDeque::new();
        let mut ret = Vec::new();
        for i in 0..n {
            if nums[i] < 0 {
                q.push_back(i);
                count[-nums[i] as usize] += 1;
            }
            if let Some(&j) = q.front() {
                if i - j >= k {
                    if nums[j] < 0 {
                        count[-nums[j] as usize] -= 1;
                    }
                    q.pop_front();
                }
            }
            if i >= k - 1 {
                let mut rank = 0;
                let mut i = 50;
                while rank < x && i > 0 {
                    rank += count[i];
                    if rank < x {
                        i -= 1;
                    }
                }
                ret.push(- (i as i32));
            }
        }
        return ret;
    }
}