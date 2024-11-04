impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut ret = String::new();
        let mut count = 0;
        let mut token = 'a';
        for c in word.chars() {
            if c == token && count < 9 {
                count += 1;
                continue;
            } 
            if count != 0 {
                ret.push((count + '0' as u8) as char);
                ret.push(token);
            }
            token = c;
            count = 1;
        }
        ret.push((count + '0' as u8) as char);
        ret.push(token);
        return ret;
    }
}
