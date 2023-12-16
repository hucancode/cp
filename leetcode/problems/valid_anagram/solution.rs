impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut a = vec![0; 1+'z' as usize];
        let mut b = vec![0; 1+'z' as usize];
        for c in s.chars() {
            a[c as usize] += 1;
        }
        for c in t.chars() {
            b[c as usize] += 1;
        }
        a.iter().zip(b.iter()).all(|(a,b)| a == b)
    }
}