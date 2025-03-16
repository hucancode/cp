impl Solution {
    pub fn repair_cars(mut ranks: Vec<i32>, cars: i32) -> i64 {
        let repair = |n| {
            ranks.iter().map(|&r| (n as f64/r as f64).sqrt() as i64).sum::<i64>()
        };
        let mut l = 0;
        let mut r = 1_00_000_000_000_000_000;
        while l < r {
            let m = (l + r)/2;
            let repaired = repair(m);
            //println!("check {l}-{r} repair({m}) = {repaired} vs {cars}");
            if repaired < cars as i64 {
                l = m+1;
            } else {
                r = m;
            }
        }
        return l;
    }
}
