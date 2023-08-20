use std::cmp::max;
impl Solution {
    pub fn maximize_the_profit(n: i32, mut offers: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut ret = 0;
        let mut f = vec![0;n];
        offers.sort_by(|a,b| a[1].cmp(&b[1]));
        let mut prev_end = 0;
        for offer in offers {
            let i = offer[0] as usize;
            let j = offer[1] as usize;
            let gold = offer[2];
            for k in prev_end..=j {
                f[k] = f[prev_end];
            }
            let prev = if i > 0 {f[i-1]} else {0};
            f[j] = max(f[j], prev + gold);
            ret = max(ret, f[j]);
            prev_end = j;
        }
        //println!("{f:?}");
        return ret;
    }
}