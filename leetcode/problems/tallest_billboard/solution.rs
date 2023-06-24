use std::collections::HashMap;
use std::cmp::max;
impl Solution {
    pub fn tallest_billboard(mut rods: Vec<i32>) -> i32 {
        let mut f = HashMap::new();
        // f[x] = what is the maximum height we can reach if the 2 rods have x different in height
        f.insert(0, 0);
        for x in rods.iter() {
            for (j,fj) in f.clone().iter() {
                let k = j + x;
                let put_high = fj + x;
                f.entry(k)
                    .and_modify(|x| *x = max(*x, put_high))
                    .or_insert(put_high);
                let k = (x - j).abs();
                let put_low = fj + max(0, x-j);
                f.entry(k)
                    .and_modify(|x| *x = max(*x, put_low))
                    .or_insert(put_low);
            }
        }
        *f.get(&0).unwrap_or(&0)
    }
}