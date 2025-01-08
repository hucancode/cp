impl Solution {
    pub fn count_prefix_suffix_pairs(mut words: Vec<String>) -> i32 {
        let mut ret = 0;
        let n = words.len();
        for i in 0..n {
            let a = &words[i];
            for j in i+1..n {
                let b = &words[j];
                if a.len() > b.len() {
                    continue;
                }
                let k = a.len();
                let matched = *a == b[..k] && *a == b[b.len()-k..];
                if matched {
                    ret += 1;
                }
            }
        }
        ret
    }
}
