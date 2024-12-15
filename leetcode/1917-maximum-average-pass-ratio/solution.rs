impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, mut extra_students: i32) -> f64 {
        use std::collections::BinaryHeap;
        let K = 1000_000_000.0;
        let mut classes: BinaryHeap<(i64, i32, i32)> = classes.into_iter()
            .map(|a| (a[0] as f64, a[1] as f64))
            .map(|(p, n)| {
                (((n-p)/n/(n+1.0)*K) as i64, p as i32, n as i32)
            })
            .collect();
        while extra_students > 0 {
            let (_, mut p, mut n) = classes.pop().unwrap();
            p += 1;
            n += 1;
            let p = p as f64;
            let n = n as f64;
            let bonus = ((n-p)/n/(n+1.0)*K) as i64;
            classes.push((bonus, p as i32, n as i32));
            extra_students -= 1;
        }
        let n = classes.len() as f64;
        classes.into_iter().map(|(_, p, n)| p as f64/n as f64).sum::<f64>() / n
    }
}
