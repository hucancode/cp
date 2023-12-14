impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        let mut v = Vec::new();
        while x > 0 {
            v.push(x%10);
            x /= 10;
        }
        x == 0 && v.iter()
            .zip(v.iter().rev())
            .all(|(a,b)| a==b)
    }
}