impl Solution {
    pub fn remove_kdigits(num: String, mut k: i32) -> String {
        let mut ret = String::new();
        for x in num.chars() {
            //println!("got {x}");
            while let Some(y) = ret.chars().last() {
                //println!("compare {x}-{y}");
                if k == 0 || x >= y {
                    break;
                }
                //println!("remove {y}");
                k -= 1;
                ret.pop();
            }
            if !(x == '0' && ret.is_empty()) {
                ret.push(x);
            }
        }
        while k > 0 && !ret.is_empty() {
            ret.pop();
            k -= 1;
        }
        if ret.is_empty() {
            ret.push('0');
        }
        ret
    }
}
