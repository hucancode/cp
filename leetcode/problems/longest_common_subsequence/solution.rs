use std::cmp::max;
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1 = text1.as_bytes();
        let text2 = text2.as_bytes();
        let n = text1.len();
        let m = text2.len();
        let mut f = vec![vec![0;m+1];n+1];
        for i in 1..=n {
            for j in 1..=m {
                f[i][j] = max(f[i-1][j], f[i][j-1]);
                if text1[i-1] == text2[j-1] {
                    f[i][j] = max(f[i][j], f[i-1][j-1] + 1);
                }
            }
        }
        return f[n][m];
    }
}