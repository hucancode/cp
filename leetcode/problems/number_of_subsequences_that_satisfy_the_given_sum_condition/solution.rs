use std::cmp::Ordering;
const INF: i64 = 1_000_000_007;
impl Solution {
    fn pow(mut x: i32, mut y: i32) -> i64 {
        if y == 0 {
            return 1;
        }
        if y == 1 {
            return x as i64;
        }
        let k = Self::pow(x, y/2) % INF;
        k * k * Self::pow(x, y%2) % INF
    }
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let n = nums.len();
        let mut ret = 0;
        println!("{nums:?}, size = {n}");
        for i in 0..n {
            let target = target - nums[i];
            let j = nums.binary_search_by(|e| match e.cmp(&target) {
                Ordering::Equal => Ordering::Less,
                ord => ord,
            }).unwrap_err() as i32 - 1;
            let i = i as i32;
            if j < i {
                break;
            }
            ret += Self::pow(2, j - i) as i32;
            ret %= INF as i32;
        }
        ret
    }
}