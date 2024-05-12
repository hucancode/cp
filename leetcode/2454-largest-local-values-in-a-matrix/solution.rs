impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::cmp::max;
        grid.windows(3)
            .map(|r3| {
                let r3 = r3.into_iter()
                    .map(|r| r.windows(3)
                        .map(|w| *w.into_iter().max().unwrap())
                        .collect::<Vec<i32>>());
                r3.reduce(|acc, r| {
                    acc.into_iter()
                        .zip(r.into_iter())
                        .map(|(a,b)| max(a,b))
                        .collect::<Vec<i32>>()
                })
                .unwrap()
            })
            .collect()
    }
}
