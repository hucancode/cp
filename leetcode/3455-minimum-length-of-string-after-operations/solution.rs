impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut count = vec![0;26];
        for c in s.chars() {
            let c = c as usize - 'a' as usize;
            count[c] += 1;
        }
        count.into_iter()
            .filter(|&c| c > 0)
            .map(|c| if c%2==0 {2} else {1})
            .sum()
    }
}
