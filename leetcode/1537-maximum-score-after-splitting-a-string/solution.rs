impl Solution {
    pub fn max_score(s: String) -> i32 {
        let n = s.len();
        let mut f = vec![0;n+1];
        for (i, c) in s.chars().enumerate() {
            f[i+1] = f[i] + if c == '0' {1} else {0};
        }
        (1..n).map(|i| n as i32 - i as i32 + f[i] + f[i] - f[n]).max().unwrap()
    }
}
