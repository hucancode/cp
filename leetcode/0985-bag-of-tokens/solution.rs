impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        tokens.sort();
        let mut score = 0;
        let mut i = 0;
        let mut n = tokens.len();
        while i < n {
            if power >= tokens[i] {
                power -= tokens[i];
                score += 1;
            } else if score > 0 {
                power += tokens[n-1] - tokens[i];
                n -= 1;
            }
            i += 1;
        }
        return score;
    }
}
