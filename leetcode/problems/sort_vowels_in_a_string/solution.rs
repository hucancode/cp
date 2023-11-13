impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut s = s.into_bytes();
        let mut idx = Vec::new();
        let mut vowels = Vec::new();
        let mut pool = "AEIOUaeiou".as_bytes();
        for (i,&c) in s.iter()
            .enumerate()
            .filter(|(_,c)| pool.binary_search(c).is_ok()) {
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