impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut f = vec![0;26];
        let count = |w: &String| {
            let mut count = vec![0;26];
            for c in w.chars() {
                let k = c as u8 - 'a' as u8;
                count[k as usize] += 1;
            }
            count
        };
        for w in words2 {
            let count = count(&w);
            for i in 0..26 {
                f[i] = f[i].max(count[i]);
            }
        }
        words1.into_iter()
            .filter(|w| count(&w)
                .iter()
                .zip(f.iter())
                .all(|(a,b)| a >= b))
            .collect()
    }
}
