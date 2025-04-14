impl Solution {
    pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        let n = n as i64;
        let k = k as i64;
        let m = m as i64;
        let MOD = 1000_000_007;
        let pow = |mut x, mut n| {
            let mut ret = 1;
            while n > 0 {
                if n%2 == 1 {
                    ret = ret*x%MOD;
                }
                x = x*x%MOD;
                n /= 2;
            }
            ret
        };
        let factorial = |n| (2..=n).fold(1, |acc, x| acc*x%MOD);
        let choose = |n, r| factorial(n)*pow(factorial(n-r), MOD-2)%MOD*pow(factorial(r), MOD-2)%MOD;
        (choose(n-1, k)*m%MOD*pow(m-1, n-1-k)%MOD) as i32
    }
}
/*
O(nk) solution
impl Solution {
    pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let m = m as i64;
        let mut f = vec![0i64;k+1];
        f[0] = m;
        for i in 1..n {
            for j in (1..=k).rev() {
                f[j] *= m-1;
                f[j] += f[j-1];
                f[j] %= 1000_000_007;
            }
            f[0] *= m-1;
            f[0] %= 1000_000_007;
            //println!("f[{i}] = {f:?}");
        }
        return f[k] as i32;
    }
}
*/
// let n = 8, k = 3
// let x = m-1
// f[0]      f[1]      f[2]       f[3]
// 1*m*x^0   0         0          0
// 1*m*x^1   1*m*x^0   0          0
// 1*m*x^2   2*m*x^1   1*m*x^0    0
// 1*m*x^3   3*m*x^2   3*m*x^1    1*m*x^0
// 1*m*x^4   4*m*x^3   6*m*x^2    4*m*x^1
// 1*m*x^5   5*m*x^4   10*m*x^3   10*m*x^2
// 1*m*x^6   6*m*x^5   15*m*x^4   20*m*x^3
// 1*m*x^7   7*m*x^6   21*m*x^5   35*m*x^4
//
// let m = 3, ans = 35*3*2^4
// 1 0 0  0
// 1 1 0  0
// 1 2 1  0
// 1 3 3  1
// 1 4 6  4
// 1 5 10 10
// 1 6 15 20
// 1 7 21 35
