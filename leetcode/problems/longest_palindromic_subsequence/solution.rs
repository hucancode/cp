use std::cmp::max;
use std::cmp::min;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let mut ret = 1;
        let n = s.len();
        let mut f = vec![vec![0;n+1];n+1];
        for (i, a) in s.chars().enumerate() {
            let i = i + 1;
            for (j, b) in s.chars().rev().enumerate() {
                let j = j + 1;
                if i + j > n + 1 {
                    continue;
                }
                if a == b {
                    let score = if i + j == n + 1 {1} else {2};
                    f[i][j] = f[i-1][j-1] + score;
                } else {
                    f[i][j] = max(f[i-1][j], f[i][j-1]);
                }
                ret = max(ret, f[i][j]);
            }
        }
        return ret;
    }
}