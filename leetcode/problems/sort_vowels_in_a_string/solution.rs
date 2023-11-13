impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut s = s.into_bytes();
        let mut idx = Vec::new();
        let mut vowels = Vec::new();
        let mut is_vowel = vec![false;256];
        for &c in "AEIOUaeiou".as_bytes() {
            is_vowel[c as usize] = true;
        }
        for (i,&c) in s.iter()
            .enumerate()
            .filter(|(_,&c)| is_vowel[c as usize]) {
            idx.push(i);
            vowels.push(c);
        }
        vowels.sort();
        for (i,c) in idx.into_iter().zip(vowels.into_iter()) {
            s[i] = c;
        }
        String::from_utf8(s.into()).unwrap()
    }
}