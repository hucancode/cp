impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let n = s1.len();
        let m = s2.len();
        let mut f = vec![0;m];
        for j in 0..m {
            let mut k = 0;
            for i in 0..n {
                if s1[i] == s2[(j+k)%m] {
                    k += 1;
                }
            }
            f[j] = k;
        }
        let mut j = 0;
        for i in 0..n1 {
            j += f[j%m];
        }
        return ((j/m) as i32)/n2;
    }
}