use std::cmp::max;
impl Solution {
    pub fn number_of_sequence(n: i32, mut sick: Vec<i32>) -> i32 {
        const MOD: i64 = 1000_000_007;
        let fast_pow = |mut n: i64, mut p: i64| {
            if p == 0 {
                return 1;
            }
            let mut ret = 1;
            while p > 1 {
                if p % 2 != 0 {
                    ret = ret * n % MOD;
                    p -= 1;
                }
                n = n * n % MOD;
                p /= 2;
            }
            ret * n % MOD
        };
        let n = n as usize;
        let m = n - sick.len();
        let mut fact = vec![1i64; 1 + n];
        for i in 2..=n {
            fact[i] = fact[i - 1] * i as i64 % MOD;
        }
        let x = sick
            .windows(2)
            .map(|a| max(1, a[1] - a[0] - 1) as i64)
            .fold(fact[m], |acc, x| acc * fast_pow(2, x - 1) % MOD);
        sick.insert(0, -1);
        sick.push(n as i32);
        let y = sick
            .windows(2)
            .map(|a| max(1, a[1] - a[0] - 1) as usize)
            .fold(1, |acc, x| acc * fact[x] % MOD);
        //println!("{}/{}", x, y);
        (x * fast_pow(y, MOD - 2) % MOD) as i32
    }
}
