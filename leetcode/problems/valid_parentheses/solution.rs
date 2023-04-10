impl Solution {
    pub fn is_valid(s: String) -> bool {
        let open = vec!['(', '{', '['];
        let close = vec![')', '}', ']'];
        let mut f = vec![];
        for c in s.chars() {
            if open.contains(&c) {
                f.push(c);
                continue;
            }
            let a = f.pop()
                .and_then(|prev| open.iter().position(|&x| x == prev));
            let b = close.iter().position(|&x| x == c);
            let matched = match (a, b) {
                (Some(i), Some(j)) => i == j,
                _ => false
            };
            if !matched {
                return false;
            }
        }
        return f.is_empty();
    }
}