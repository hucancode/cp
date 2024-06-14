impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut target = 0;
        let mut ret = 0;
        for x in nums {
            ret += std::cmp::max(0, target - x);
            target = std::cmp::max(target,x)+1;
        }
        ret
    }
}
