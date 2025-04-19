impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort();
        let mut ret = 0;
        for (i, &x) in nums.iter().enumerate() {
            let l = nums[..i].partition_point(|&y| y < lower - x);
            let r = nums[..i].partition_point(|&y| y <= upper - x);
            ret += (r - l) as i64;
        }
        return ret;
    }
}
