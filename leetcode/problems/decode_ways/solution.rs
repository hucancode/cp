impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let n = s.len();
        let s = s.as_bytes();
        let mut f = vec![0;n+1];
        f[0] = 1;
        if s[0] != '0' as u8 {
            f[1] = 1;
        }
        for i in 2..=n {
            let a = s[i-1] - '0' as u8;
            let b = s[i-2] - '0' as u8;
            if a >= 1 && a <= 9 {
                f[i] += f[i-1];
            }
            if b == 1 || (b == 2 && a <= 6) {
                f[i] += f[i-2];
            }
        }
        return f[n];
    }
}