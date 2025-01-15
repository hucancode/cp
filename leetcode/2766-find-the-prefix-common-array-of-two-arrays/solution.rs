impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let n = a.len();
        let mut vis = HashSet::new();
        let mut ret = 0;
        a.into_iter()
            .zip(b.into_iter())
            .map(|(a,b)| {
                if !vis.insert(a) {
                    ret += 1;
                }
                if !vis.insert(b) {
                    ret += 1;
                }
                ret
            })
            .collect()
    }
}
