use std::collections::HashMap;
use std::cmp::max;
const BIT_PER_CHAR: usize = 5;
struct WordFilter {
    dict: HashMap<(i64,i64), i32>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {

    fn new(words: Vec<String>) -> Self {
        let mut dict = HashMap::new();
        for (idx, w) in words.into_iter().enumerate() {
            let n = w.len();
            let w: Vec<i64> = w.as_bytes()
                .into_iter()
                .map(|x| x - 'a' as u8 + 1)
                .map(|x| x as i64)
                .collect();
            let mut prefix = vec![0;n];
            let mut suffix = vec![0;n];
            prefix[0] = w[0];
            for i in 1..n {
                prefix[i] = prefix[i-1];
                prefix[i] += w[i]<<(i*BIT_PER_CHAR);
            }
            suffix[n-1] = w[n-1];
            for i in (0..n-1).rev() {
                suffix[i] = suffix[i+1];
                suffix[i] <<= BIT_PER_CHAR;
                suffix[i] += w[i];
            }
            for i in 0..n {
                for j in 0..n {
                    dict.insert((prefix[i], suffix[j]), idx as i32);
                }
            }
        }
        //println!("{dict:?}");
        Self {
            dict: dict
        }
    }

    fn hash(input: String) -> i64 {
        let mut ret = 0;
        for x in input.as_bytes()
            .into_iter()
            .rev()
            .map(|x| x - 'a' as u8 + 1)
            .map(|x| x as i64) {
            ret <<= BIT_PER_CHAR;
            ret += x;
        }
        return ret;
    }
    
    fn f(&self, pref: String, suff: String) -> i32 {
        let key = (Self::hash(pref), Self::hash(suff));
        let ret = *self.dict.get(&key).unwrap_or(&-1);
        return ret;
    }
}

/**
 * Your WordFilter object will be instantiated and called as such:
 * let obj = WordFilter::new(words);
 * let ret_1: i32 = obj.f(pref, suff);
 */