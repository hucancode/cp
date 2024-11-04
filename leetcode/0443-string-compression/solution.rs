impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut token = '\0';
        let mut count = 0;
        let cost = |count| {
            if count < 1 {0}
            else if count < 2 {1}
            else if count < 10 {2}
            else if count < 100 {3}
            else if count < 1000 {4}
            else {5}
        };
        let mut put_counter = |mut count: i32, j: &mut usize, chars: &mut Vec<char>| {
            let mut n = if count < 2 {0}
            else if count < 10 {1}
            else if count < 100 {10}
            else if count < 1000 {100}
            else {1000};
            while n > 0 {
                chars[*j] = ((count/n) as u8 + '0' as u8) as char;
                count %= n;
                n /= 10;
                *j += 1;
            }
        };
        let mut ret = 0;
        let mut i = 0;
        let mut j = 0;
        for i in 0..chars.len() {
            if chars[i] == token {
                count += 1;
                continue;
            }
            ret += cost(count);
            put_counter(count, &mut j, chars);
            count = 1;
            token = chars[i];
            chars[j] = token;
            j += 1;
        }
        ret += cost(count);
        put_counter(count, &mut j, chars);
        return ret;
    }
}
