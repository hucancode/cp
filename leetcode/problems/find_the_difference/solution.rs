impl Solution {
    pub fn find_the_difference(mut s: String, t: String) -> char {
        let mut f = vec![0;26];
        let mut g = vec![0;26];
        for c in s.chars() {
            let i = c as usize - 'a' as usize;
            f[i] += 1;
        }
        for c in t.chars() {
            let i = c as usize - 'a' as usize;
            g[i] += 1;
            if g[i] > f[i] {
                return c;
            }
        }
        return '_';
    }
}