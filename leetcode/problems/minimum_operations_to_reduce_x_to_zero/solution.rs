impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, x: i32) -> i32 {
        let n = nums.len();
        let mut f = vec![0];
        f.append(&mut nums);
        for i in 1..=n {
            f[i] += f[i-1];
        }
        if x > f[n] {
            return -1;
        }
        let mut ret = n+1;
        for i in 0..n {
            let target = x - f[i];
            if let Ok(j) = f.binary_search_by(|y| {
                let y = f[n] - y;
                return target.cmp(&y);
            }) {
                ret = std::cmp::min(ret, i+n-j);
            };
        }
        if ret > n {-1} else {ret as i32}
    }
}