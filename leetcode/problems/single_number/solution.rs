impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        for x in nums {
            ret ^= x;
        }
        ret
    }
}