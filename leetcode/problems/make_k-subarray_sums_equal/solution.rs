use std::cmp::min;

impl Solution {
    pub fn make_sub_k_sum_equal(arr: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let n = arr.len();
        let gcd = |mut x, mut y| {
            while y != 0 {
                let z = x%y;
                x = y;
                y = z;
            }
            return x;
        };
        let stride = gcd(n, k);
        let mut ret = 0;
        for x in 0..stride {
            let mut sub: Vec<i32> = arr
                .clone()
                .into_iter()
                .skip(x)
                .step_by(stride)
                .collect();
            sub.sort();
            let median = sub[sub.len()/2];
            let needed = sub
                .iter()
                .fold(0, |acc, x| acc + (x-median).abs() as i64);
            //println!("balancing {sub:?} need {needed}");
            ret += needed;
        }
        return ret;
    }
}