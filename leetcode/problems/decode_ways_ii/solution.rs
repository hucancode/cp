impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let INF = 1000_000_007i64;
        let n = s.len();
        let s = s.as_bytes();
        let mut f = vec![0i64;n+1];
        f[0] = 1;
        if s[0] == '*' as u8 {
            f[1] = 9;
        } else if s[0] != '0' as u8 {
            f[1] = 1;
        }
        for i in 2..=n {
            let a = s[i-1] as char;
            let b = s[i-2] as char;
            f[i] += match a {
                '0'     => 0,
                '*'     => f[i-1]*9,
                _       => f[i-1],
            };
            f[i] %= INF;
            f[i] += match (b,a) {
                ('1', '*')          => f[i-2]*9,
                ('1', _)            => f[i-2],
                ('2', '*')          => f[i-2]*6,
                ('2', '0'..='6')    => f[i-2],
                ('*', '*')          => f[i-2]*15,
                ('*', '0'..='6')    => f[i-2]*2,
                ('*', _)            => f[i-2],
                _                   => 0,
            };
            f[i] %= INF;
        }
        //println!("{f:?}");
        return f[n] as i32;
    }
}