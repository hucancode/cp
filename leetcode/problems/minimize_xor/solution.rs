impl Solution {
    pub fn minimize_xor(mut num1: i32, num2: i32) -> i32 {
        let mut i = 0;
        while num1.count_ones() < num2.count_ones() {
            num1 |= 1<<i;
            i += 1;
        }
        i = 0;
        while num1.count_ones() > num2.count_ones() {
            num1 &= !(1<<i);
            i += 1;
        }
        num1
    }
}