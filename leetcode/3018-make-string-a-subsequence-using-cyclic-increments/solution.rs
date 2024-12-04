impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let s1 = str1.as_bytes();
        let s2 = str2.as_bytes();
        let n = s1.len();
        let m = s2.len();
        let mut i = 0;
        let mut j = 0;
        while i < n && j < m {
            let a1 = s1[i] - 'a' as u8;
            let a2 = (a1 + 1)%26;
            let b = s2[j] - 'a' as u8;
            if a1 == b || a2 == b {
                j += 1;
            }
            i+= 1;
        }
        return j == m;
    }
}
