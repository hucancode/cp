impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        let solve = |a: Vec<i32>| {
            let l = a[0];
            let r = a[1];
            let mut sum = 0;
            let mut cost = 1;
            let mut k = 1;
            while k <= r {
                let count = r.min(k*4-1) - l.max(k) + 1;
                sum += (cost * count as i64).max(0);
                cost += 1;
                k *= 4;
            }
            (sum + 1)/2
        };
        queries.into_iter().map(solve).sum()
    }
}
