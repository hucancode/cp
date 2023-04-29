impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        let mut i = n as i32 - 1;
        while i >= 0 {
            let useless = nums[i as usize] > i+1 || nums[i as usize] < 1;
            if useless {
                i -= 1;
                continue;
            }
            let j = nums[i as usize] as usize - 1;
            let useless = nums[i as usize] == nums[j];
            if useless {
                i -= 1;
                continue;
            }
            let x = nums[j];
            nums[j] = nums[i as usize];
            nums[i as usize] = x;
        }
        i = 1;
        while i <= n as i32 && nums[i as usize - 1] == i {
            i += 1;
        }
        return i;
    }
}