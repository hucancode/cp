use std::collections::HashSet;
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut set = HashSet::new();
        let mut ret = Vec::new();
        let n = nums.len() as i32;
        for x in nums {
            if !set.insert(x) {
                ret.push(x);
            }
        }
        ret.extend((1..=n).filter(|i| !set.contains(&i)));
        ret
    }
}