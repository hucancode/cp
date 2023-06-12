use std::cmp::max;

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let can_ship = |cap| {
            let mut day = days;
            let mut load = 0;
            for &i in weights.iter() {
                load += i;
                if load <= cap {
                    continue;
                }
                day -= 1;
                if (day <= 0) {
                    break;
                }
                load = i;
            }
            day > 0
        };
        let mut l = weights.iter().fold(0, |acc, &x| max(acc,x));
        let mut r = weights.iter().sum::<i32>();
        // shorter code, less headache, but worse performance
        // let candidates: Vec<i32> = (l..r).collect();
        // let i = candidates.partition_point(|&cap| !can_ship(cap)) as i32;
        // return l + i;
        while l < r {
            let m = (l + r)/2;
            if can_ship(m) {
                r = m;
            } else {
                l = m + 1;
            }
        }
        return l;
    }
}