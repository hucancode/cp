impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let n = s.len();
        for i in 0..n/2 {
            let j = n-i-1;
            let c = s[i];
            s[i] = s[j];
            s[j] = c;
        }
    }
}
