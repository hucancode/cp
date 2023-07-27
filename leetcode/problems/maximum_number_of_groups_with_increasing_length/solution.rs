impl Solution {
    pub fn max_increasing_groups(mut lim: Vec<i32>) -> i32 {
        lim.sort();
        let mut ret = 0;
        let mut needed = 0;
        let mut available = 0;
        for x in lim {
            available += x as i64;
            needed = ret as i64 + 1;
            if available >= needed {
                ret += 1;
                available -= needed;
            }
        }
        return ret;
    }
}