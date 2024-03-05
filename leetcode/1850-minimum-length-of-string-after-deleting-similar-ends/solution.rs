impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let mut i = 0;
        let mut j = s.len() - 1;
        while j > i {
            if s[i] != s[j] {
                break;
            }
            let i0 = i;
            while i < j && s[i] == s[i0] {
                i += 1;
            }
            while i <= j && s[j] == s[i0] {
                j -= 1;
            }
        }
        return j as i32 - i as i32 + 1;
    }
}
