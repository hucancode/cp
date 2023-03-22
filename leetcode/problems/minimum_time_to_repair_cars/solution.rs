impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let n = cars as i64;
        let check = |t| {
            let mut repaired = 0i64;
            for &r in &ranks {
                repaired += ((t/(r as i64)) as f64).sqrt() as i64;
                if(repaired >= n) {
                    return true;
                }
            }
            return false;
        };
        let mut l = 0i64;
        let mut r = n*n*(ranks[0] as i64);
        while(l < r) {
            let m = (l+r)/2;
            if(!check(m)) {
                l = m + 1;
            } else {
                r = m;
            }
        }
        return l;
    }
}