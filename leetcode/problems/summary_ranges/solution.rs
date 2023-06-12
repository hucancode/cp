impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let n = nums.len();
        if n == 0 {
            return Vec::new();
        }
        let mut a = nums[0];
        let mut b = nums[0];
        let mut ret = Vec::new();
        for i in 1..=n {
            if i == n || nums[i] != b+1 {
                if a == b {
                    ret.push(format!("{a}"));
                } else {
                    ret.push(format!("{a}->{b}"));
                }
                if i != n {
                    a = nums[i];
                }
            }
            if i != n {
                b = nums[i];
            }
        }
        ret
    }
}