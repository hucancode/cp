impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut ret = String::new();
        let mut x = 0;
        for c in s.chars().rev() {
            if c == '*' {
                x += 1;
                continue;
            }
            if x > 0 {
                x -= 1;
                continue;
            }
            ret.push(c);
        }
        return ret.chars().rev().collect();
    }
}