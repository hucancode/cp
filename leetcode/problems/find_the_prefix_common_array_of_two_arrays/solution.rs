use std::collections::HashSet;
impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut k = 0;
        let mut ret = Vec::new();
        let mut c = HashSet::new();
        for (a,b) in a.iter().zip(b.iter()) {
            if c.contains(b) {
                k += 1;
            }
            if c.contains(a) {
                k += 1;
            }
            if (a == b) {
                k += 1;
            }
            c.insert(a);
            c.insert(b);
            ret.push(k);
        }
        ret
    }
}