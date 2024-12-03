impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut ret = String::with_capacity(s.len()+spaces.len());
        let mut i = 0;
        for j in spaces {
            ret.push_str(&s[i..j as usize]);
            ret.push(' ');
            i = j as usize;
        }
        ret.push_str(&s[i..s.len()]);
        ret
    }
}
