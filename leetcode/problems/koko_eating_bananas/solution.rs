impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut l = 1;
        let mut r = 1e9 as i32;
        let check = |k| {
            let mut cost = 0;
            for &p in &piles {
                cost += p/k + if(p%k==0) {0} else {1};
                if(cost > h) {
                    return false;
                }
            }
            return true;
        };
        while(l < r) {
            let m = (l+r)/2;
            if(check(m)) {
                r = m;
            } else {
                l = m+1;
            }
        }
        return l;
    }
}