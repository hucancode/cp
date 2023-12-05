impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let count = |s: String| {
            let mut ret = vec![0;26];
            s.chars()
                .map(|c| c as usize - 'a' as usize)
                .for_each(|c| ret[c] += 1);
            ret
        };
        let pool = count(chars);
        words.into_iter()
            .filter_map(|w| {
                let ret = w.len() as i32;
                let count = count(w);
                if count.iter().zip(pool.iter()).all(|(a,b)| a <= b) {
                    Some(ret)
                } else {
                    None
                }
            })
            .sum::<i32>()
    }
}