use std::cmp::max;
impl Solution {
    pub fn delete_string(s: String) -> i32 {
        let n = s.len();
        let mut f = vec![1;n];
        for l in (0..n).rev() {
            let k = (n-l)/2;
            for len in 1..=k {
                let m = l+len;
                let r = m+len;
                //println!("l = {l}, m = {m}, r = {r}");
                let a = &s[l..m];
                let b = &s[m..r];
                //println!("{a}-{b}");
                if(a == b) {
                    f[l] = max(f[l], f[m]+1);
                }
            }
        }
        //println!("{f:?}");
        return *f.first().unwrap_or(&1);
    }
}