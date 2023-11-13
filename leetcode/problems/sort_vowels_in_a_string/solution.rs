impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut s = s.into_bytes();
        let mut idx = Vec::new();
        let mut vowels = Vec::new();
        let pool = "aeiouAEIOU".as_bytes();
        for (i,&c) in s.iter()
            .enumerate()
            .filter(|(_,&c)| {
                pool.iter().any(|&x| c == x)
            }) {
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