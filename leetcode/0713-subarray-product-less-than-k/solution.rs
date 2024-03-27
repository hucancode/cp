impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 0;
        }
        let mut acc = 1;
        let mut q = std::collections::VecDeque::new();
        let mut ret = 0;
        for x in nums {
            q.push_back(x);
            acc *= x;
            while acc >= k {
                if let Some(x) = q.pop_front() {
                    acc /= x;
                }
            }
            ret += q.len();
        }
        return ret as i32;
    }
}
