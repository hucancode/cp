impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let n = s.len();
        if n < 2 {
            return 0;
        }
        let s = s.as_bytes();
        let mut p = vec![0;n+1];
        let mut q = vec![0;n+1];
        for i in 1..=n {
            p[i] = p[i-1];
            q[i] = q[i-1];
            if i%2 == 0 && s[i-1] == '1' as u8 {
                p[i] += 1;
            }
            if i%2 != 0 && s[i-1] == '1' as u8 {
                q[i] += 1;
            }
        }
        let pn = n/2;
        let qn = (n+1)/2;
        let p1q0 = pn - p[n] + q[n];
        let p0q1 = p[n] + qn - q[n];
        std::cmp::min(p1q0, p0q1) as i32
    }
}