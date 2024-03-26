impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in 0..n {
            let mut x = nums[i];
            nums[i] = 0;
            while x >= 1 && x as usize <= n && nums[x as usize - 1] != x {
                let next = nums[x as usize - 1];
                nums[x as usize - 1] = x;
                x = next;
            }
        }
        let mut ret = 1;
        while ret as usize <= n && nums[ret as usize - 1] == ret {
            ret += 1;
        }
        ret
    }
}
