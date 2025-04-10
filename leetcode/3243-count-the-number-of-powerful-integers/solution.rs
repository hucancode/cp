impl Solution {
    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        let s: i64 = s.parse().unwrap();
        let count = |mut n: i64, limit: i64, mut s: i64| {
            if n < s {
                return 0;
            }
            if s == 0 {
                n /= 10;
            }
            let mut can_use_tail = true;
            while s > 0 {
                let a = n%10;
                let b = s%10;
                if a < b {
                    can_use_tail = false;
                } else if a > b {
                    can_use_tail = true;
                }
                s /= 10;
                n /= 10;
            }
            if !can_use_tail {
                n -= 1;
            }
            //println!("start with n = {n}");
            let mut ret = 1;
            let mut k = 1i64;
            while n > 0 {
                let d = n%10;
                if d > limit {
                    ret = (limit + 1)*k;
                } else {
                    ret += d.min(limit)*k;
                }
                k *= limit + 1;
                //println!("n={n}, ret = {ret}");
                n /= 10;
            }
            //println!("return {ret}");
            return ret;
        };
        return count(finish, limit as i64, s) - count(start-1, limit as i64, s);
    }
}
