impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut d = 0;
        let mut ret = 0;
        for c in s.chars() {
            d += match c {
                '(' => 1,
                ')' => -1,
                _ => 0
            };
            ret = std::cmp::max(ret, d);
        }
        ret
    }
}
