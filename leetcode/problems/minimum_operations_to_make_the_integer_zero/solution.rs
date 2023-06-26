impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        let num1 = num1 as i64;
        let num2 = num2 as i64;
        // haven't figured out which is the upper bound of the answer
        const n: i64 = 1000;
        for i in 1..n {
            let x = num1 - num2*i;
            // with only i 1-bit elements, can we build an array has sum = x ???
            // turns out with i 1-bit elements, 
            // we can make any sum between [i, j] where j = maximum i-bit integer
            if x.count_ones() <= i as u32 && i <= x {
                return i as i32;
            }
        }
        return -1;
    }
}