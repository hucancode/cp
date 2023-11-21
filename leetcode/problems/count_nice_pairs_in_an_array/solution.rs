use std::collections::HashMap;
impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        const INF: i64 = 1000_000_007;
        let rev = |mut x: i64| {
            let mut ret: i64 = 0;
            while x > 0 {
                ret *= 10;
                ret += x%10;
                x /= 10;
            }
            return ret;
        };
        let mut delta_count: HashMap<i64, i64> = HashMap::new();
        for x in nums {
            let d = x as i64 - rev(x as i64);
            *delta_count.entry(d).or_default() += 1;
        }
        //println!("{:?}", delta);
        delta_count
            .into_values()
            .fold(0, |acc,n| {
                if n < 2 {
                    acc
                } else {
                    (acc + (n-1)*n/2)%INF
                }
            }) as i32
    }
}