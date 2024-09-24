impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut s1 = HashSet::new();
        let mut s2 = HashSet::new();
        for mut x in arr1 {
            while x > 0 {
                s1.insert(x);
                x /= 10;
            }
        }
        for mut x in arr2 {
            while x > 0 {
                s2.insert(x);
                x /= 10;
            }
        }
        let mut ret = 0;
        if let Some(&x) = s1.intersection(&s2).max() {
            let mut x = x;
            while x > 0 {
                ret += 1;
                x /= 10;
            }
        }
        return ret;
    }
}
