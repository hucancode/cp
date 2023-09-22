impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let n = t.len();
        let m = s.len();
        let mut i = 0;
        let mut j = 0;
        while i<n && j<m {
            if t[i] == s[j] {
                j+=1;
            }
            i+= 1;
        }
        return j >= m;
    }
}