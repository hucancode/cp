impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let gcd = |mut a, mut b| {
            while b != 0 {
                let r = a%b;
                a = b;
                b = r;
            }
            return a;
        };
        let n = nums.len();
        let k = k as usize;
        let mut ret = 0;
        let mut gcds: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..n {
            let x = gcd(i, k);
            for (&y, ids) in gcds.iter() {
                if x*y%k != 0 {
                    continue;
                }
                for &j in ids.iter() {
                    if nums[i] == nums[j] {
                        ret += 1;
                    }
                }
            }
            gcds.entry(x).or_default().push(i);
        }
        return ret;
    }
}
