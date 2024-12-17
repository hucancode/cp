impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        use std::collections::HashMap;
        let mut occ: HashMap<char, Vec<usize>> = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            occ.entry(c).or_default().push(i);
        }
        occ.into_values()
            .filter_map(|a| {
                let mut last = 0;
                let mut streak = 0;
                let mut candidates = Vec::new();
                for i in a {
                    if i == last + 1 {
                        streak += 1;
                    } else {
                        streak = 1;
                    }
                    candidates.push(streak);
                    last = i;
                }
                candidates.sort_by(|a,b| b.cmp(&a));
                candidates.into_iter().skip(2).next()
            })
            .max()
            .unwrap_or(-1)
    }
}
