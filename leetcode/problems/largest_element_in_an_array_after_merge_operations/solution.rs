impl Solution {
    pub fn max_array_value(nums: Vec<i32>) -> i64 {
        let mut ret = 0;
        let mut x = 0;
        for &i in nums.iter().rev() {
            if x < i as i64 {
                x = i as i64;
            } else {
                x += i as i64;
            }
            ret = std::cmp::max(ret, x);
        }
        return ret;
    }
}