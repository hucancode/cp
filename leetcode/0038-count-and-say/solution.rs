impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let rle = |s: String| {
            let mut ret = String::new();
            let mut n = 0;
            let mut last = '_';
            for c in s.chars() {
                if c != last && last != '_' {
                    ret.push_str(&n.to_string());
                    ret.push(last);
                    n = 0;
                }
                last = c;
                n += 1;
            }
            ret.push_str(&n.to_string());
            ret.push(last);
            //println!("{ret}");
            return ret;
        };
        let mut ret = String::from("1");
        for i in 1..n {
            ret = rle(ret);
        }
        return ret;
    }
}
