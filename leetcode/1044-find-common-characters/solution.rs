impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        use std::cmp::min;
        let mut f = vec![usize::MAX;26];
        for w in words {
            let mut count = vec![0;26];
            for c in w.chars() {
                count[(c as u8 - 'a' as u8) as usize] += 1;
            }
            for (a, b) in f.iter_mut().zip(count.iter()) {
                *a = min(*a,*b);
            }
        }
        let mut ret = Vec::new();
        let mut c = 'a' as u8;
        for count in f {
            for i in 0..count {
                ret.push(String::from(c as char));
            }
            c += 1;
        }
        ret
    }
}
