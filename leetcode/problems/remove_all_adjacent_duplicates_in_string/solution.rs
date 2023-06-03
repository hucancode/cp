impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut ret = String::new();
        for c in s.chars() {
            if ret.chars().last().filter(|&x| c==x).is_some() {
                ret.pop();
            } else {
                ret.push(c);
            }
        }
        ret
    }
}