impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by(|a,b| a[0].cmp(&b[0]));
        (1..points.len())
            .map(|i| points[i][0] - points[i-1][0])
            .max()
            .unwrap_or(0)
    }
}