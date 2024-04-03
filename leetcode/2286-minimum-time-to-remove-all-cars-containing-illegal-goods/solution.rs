impl Solution {
    pub fn minimum_time(s: String) -> i32 {
        use std::cmp::min;
        let n = s.len();
        let mut pick_mid = 0;
        let mut clear_left = 0;
        let mut ret = n;
        for (i,c) in s.chars().enumerate() {
            let clear_right = n-i-1;
            if c == '1' {
                pick_mid = min(clear_left, pick_mid) + 2;
                clear_left = i+1;
            }
            ret = min(ret, min(pick_mid, clear_left) + clear_right);
        }
        ret as i32
    }
}
