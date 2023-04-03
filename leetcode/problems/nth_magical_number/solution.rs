impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        const INF: i64 = 1e9 as i64 + 7;
        let a = a as i64;
        let b = b as i64;
        let n = n as i64;
        let gcd = |mut x,mut y| {
            while y != 0 {
                let z = x%y;
                x = y;
                y = z;
            }
            return x;
        };
        let lcm = |x,y| x*y / gcd(x,y);
        let c = lcm(a, b);
        let mut l = 1 as i64;
        let mut r = 4e13 as i64;
        while l < r {
            let m = (l+r)/2;
            let k = m/a + m/b - m/c;
            if k < n {
                l = m+1;
            } else {
                r = m;
            }
        }
        let ret = l % INF;
        return ret as i32;
    }
}