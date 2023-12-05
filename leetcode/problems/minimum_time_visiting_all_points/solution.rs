impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        points.windows(2)
            .map(|ab| {
                let dx = ab[0][0] - ab[1][0];
                let dy = ab[0][1] - ab[1][1];
                dx.abs().max(dy.abs())
            })
            .sum()
    }
}