impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let n = score.len();
        let medals = [
            "Gold Medal",
            "Silver Medal",
            "Bronze Medal",
        ];
        let mut input: Vec<(i32, usize)> = score.into_iter().enumerate().map(|(i,x)| (x,i))
            .collect();
        input.sort();
        let mut output = vec![String::new();n];
        input.into_iter()
            .rev()
            .enumerate()
            .for_each(|(i,(_,j))| {
                output[j] = match i {
                    0 => "Gold Medal".to_string(),
                    1 => "Silver Medal".to_string(),
                    2 => "Bronze Medal".to_string(),
                    _ => (i+1).to_string(),
                };
            });
        output
    }
}
