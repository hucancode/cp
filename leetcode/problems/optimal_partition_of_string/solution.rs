impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut f = vec![false;26];
        let mut ret = 1;
        for c in s.chars() {
            let i = c as usize - 'a' as usize;
            if f[i] {
                ret += 1;
                f.fill(false);
            }
            f[i] = true;
        }
        return ret;
    }
}