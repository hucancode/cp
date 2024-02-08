impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut pool = Vec::new();
        let mut i = 1;
        while i*i <= n {
            pool.push(i*i);
            i += 1;
        }
        let mut f = vec![n; 1+n];
        f[0] = 0;
        for i in 0..n {
            for x in pool.iter() {
                if i + x > n {
                    break;
                }
                f[i+x] = std::cmp::min(f[i+x], f[i] + 1);
            }
        }
        return f[n] as i32;
    }
}