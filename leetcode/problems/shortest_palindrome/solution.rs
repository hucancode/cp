impl Solution {
    pub fn shortest_palindrome(mut s: String) -> String {
        let a: Vec<char> = s.chars().collect();
        let n = a.len();
        let mut keep = (1..=n).rev()
            .find(|i| (0..i/2).all(|j| a[j] == a[i-j-1]))
            .unwrap_or(1);
        let mut b: Vec<char> = a.iter()
            .skip(keep)
            .rev()
            .cloned()
            .collect();
        b.extend(a);
        b.into_iter()
            .collect()
    }
}