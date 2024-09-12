impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut jump = 0;
        let mut distance = 0;
        let mut next = 0;
        for i in 0..n {
            if i > distance {
                jump += 1;
                distance = next;
            }
            next = std::cmp::max(next, i+nums[i] as usize);
        }
        return jump;
    }
}
