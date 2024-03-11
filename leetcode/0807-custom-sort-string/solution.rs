impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut count = vec![0;26];
        for c in s.chars() {
            let c = c as usize - 'a' as usize;
            count[c] += 1;
        }
        let mut ret = String::new();
        for c in order.chars() {
            let k = c as usize - 'a' as usize;
            for i in 0..count[k] {
                ret.push(c);
            }
            count[k] = 0;
        }
        for i in 0..26 {
            let c = (i as u8 + 'a' as u8) as char;
            for j in 0..count[i] {
                ret.push(c);
            }
        }
        ret
    }
}
