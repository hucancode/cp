impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        position.sort();
        let valid = |i: i32| {
            let mut last = -i;
            let mut count = 0;
            for &x in position.iter() {
                if x - last >= i {
                    count += 1;
                    last = x;
                }
            }
            return count >= m;
        };
        let mut l = 1;
        let mut r = position[position.len() - 1] - position[0] + 1;
        while l < r {
            let m = (l+r)/2;
            if valid(m) {
                l = m+1;
            } else {
                r = m;
            }
        }
        return r - 1;
    }
}
