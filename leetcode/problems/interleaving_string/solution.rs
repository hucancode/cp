impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        let n = s1.len();
        let m = s2.len();
        let nm = s3.len();
        if n+m != nm {
            return false;
        }
        let mut f = vec![false;m+1];
        for i in 0..=n {
            for j in 0..=m {
                f[j] = (i == 0 && j == 0) || 
                    (i > 0 && f[j] && s1[i-1] == s3[i+j-1]) ||
                    (j > 0 && f[j-1] && s2[j-1] == s3[i+j-1]);
            }
        }
        return f[m];
    }
}