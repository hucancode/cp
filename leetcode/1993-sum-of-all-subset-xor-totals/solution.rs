impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = 1<<n;
        let mut ret = 0;
        for mask in 1..m {
            let mut x = 0;
            for i in (0..n).filter(|i| (1<<i) & mask != 0) {
                x ^= nums[i];
            }
            ret += x;
        }
        ret
    }
}
