impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut count = vec![0;256];
        for c in s.chars() {
            count[c as usize] += 1;
        }
        let mut ret: i32 = count.iter().map(|c| c/2*2).sum();
        if count.iter().any(|c| c%2 == 1) {
            ret += 1;
        }
        return ret;
    }
}
