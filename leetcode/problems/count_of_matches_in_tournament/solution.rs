impl Solution {
    pub fn number_of_matches(mut n: i32) -> i32 {
        let mut ret = 0;
        while n > 1 {
            ret += n/2;
            n = n/2+n%2;
        }
        ret
    }
}