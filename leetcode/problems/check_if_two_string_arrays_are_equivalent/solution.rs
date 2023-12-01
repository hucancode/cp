impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let w1 = word1.iter()
            .map(|s| s.chars())
            .flatten();
        let w2 = word2.iter()
            .map(|s| s.chars())
            .flatten();
        w1.clone().count() == w2.clone().count() && 
            w1.zip(w2).all(|(a,b)| a == b)
    }
}