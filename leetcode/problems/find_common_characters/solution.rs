use std::cmp::min;
impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        words.into_iter()
            .map(|s| {
                let mut ret = vec![0;26];
                s.chars()
                    .map(|c| c as usize - 'a' as usize)
                    .for_each(|i| ret[i] += 1);
                ret
            })
            .reduce(|a, b| a.into_iter()
                .zip(b.into_iter())
                .map(|(a,b)| min(a,b))
                .collect())
            .unwrap_or(vec![])
            .into_iter()
            .enumerate()
            .map(|(i, n)| vec![i;n])
            .flatten()
            .map(|i| i as u8 + 'a' as u8)
            .map(char::from)
            .map(String::from)
            .collect()
    }
}