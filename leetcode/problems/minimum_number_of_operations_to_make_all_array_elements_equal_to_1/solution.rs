use std::cmp::min;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let count = nums.iter().filter(|&&x| x == 1).count();
        if count != 0 {
            return (n-count) as i32;
        }
        let gcd = |mut a,mut b| {
            while b != 0 {
                let c = b;
                b = a%b;
                a = c;
            }
            a
        };
        const INF:i32 = 1000_000_000;
        let mut best = INF;
        for i in 0..n {
            let mut k = nums[i];
            let len = nums.iter()
                .skip(i)
                .take_while(|&&x| {
                    k = gcd(k, x);
                    k != 1
                })
                .count();
            if k == 1 {
                best = min(best, (n-1+len) as i32);
            }
        }
        if best != INF {best} else {-1}
    }
}