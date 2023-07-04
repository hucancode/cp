impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut appear1 = 0;
        let mut appear2 = 0;
        for x in nums.iter() {
            appear1 ^= x & !appear2;
            appear2 ^= x & !appear1;
        }
        appear1
    }
}