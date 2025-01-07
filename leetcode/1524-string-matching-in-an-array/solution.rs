impl Solution {
    pub fn string_matching(mut words: Vec<String>) -> Vec<String> {
        let n = words.len();
        words.sort_by(|a,b| a.len().cmp(&b.len()));
        //println!("{words:?}");
        for i in 0..n {
            let needle = &words[i];
            if words[i+1..n].iter()
                .all(|haysack| haysack.find(needle).is_none()) {
                words[i].clear();
            }
        }
        words.into_iter()
            .filter(|w| !w.is_empty())
            .collect()
    }
}
