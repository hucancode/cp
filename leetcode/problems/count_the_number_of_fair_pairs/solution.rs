impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut nums = nums.clone();
        let mut ret = 0i64;
        nums.sort();
        for (i,x) in nums.iter().enumerate() {
            let j = nums[i+1..].partition_point(|&y| (x + y) < lower);
            let j = nums[i+1+j..].partition_point(|&y| (x + y) <= upper);
            ret += j as i64;
        }
        return ret;
    }
}