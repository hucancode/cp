impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i64 {
        use std::collections::HashMap;
        let gcd = |mut a, mut b| {
            while b != 0 {
                let r = a%b;
                a = b;
                b = r;
            }
            return a;
        };
        let k = k as i64;
        let mut ret = 0;
        let mut gcds: HashMap<i64, i64> = HashMap::new();
        for x in nums {
            let x = gcd(x as i64, k);
            ret += gcds.iter()
                .filter(|&(&y, _)| x * y % k == 0)
                .map(|(_, &count)| count)
                .sum::<i64>();
            *gcds.entry(x).or_default() += 1;
        }
        return ret;
    }
}
