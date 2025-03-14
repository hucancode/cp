impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let n = candies.len();
        let count_children = |x| {
            if x == 0 {
                return i64::MAX;
            }
            candies.iter().map(|c| (c/x) as i64).sum::<i64>()
        };
        let mut l = 0;
        let mut r = *candies.iter().max().unwrap();
        while l < r {
            let m = (l + r + 1)/2;
            //println!("search {l}-{r}, can give {m} x {} candies", count_children(m));
            if count_children(m) >= k {
                l = m;
            } else {
                r = m-1;
            }
        }
        return l;
    }
}
