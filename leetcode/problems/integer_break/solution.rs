use std::cmp::max;
impl Solution {
    pub fn pow(a: i32, x: i32) -> i32 {
        if x == 0 {
            return 1;
        } else if x == 1 {
            return a;
        }
        let y = Self::pow(a,x/2);
        let z = Self::pow(a,x%2);
        return y*y*z;
    }
    pub fn integer_break(n: i32) -> i32 {
        let mut ret = 1;
        for k in 2..n {
            let r = n%k;
            let x = n/k;
            let y = x+1;
            let score = Self::pow(x,k-r)*Self::pow(y,r);
            ret = max(ret, score);
        }
        return ret;
    }
}