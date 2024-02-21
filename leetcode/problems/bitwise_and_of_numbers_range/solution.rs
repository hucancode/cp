impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut ret = 0;
        for i in 0..31 {
            let k = 1<<i;
            let x = left/k;
            if x%2 == 0 {
                continue;
            }
            let y = right/k;
            if x != y {
                continue;
            }
            ret |= k;
        }
        ret
    }
}