impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by(|a,b| a[0].cmp(&b[0]));
        points.windows(2)
            .map(|a| a[1][0] - a[0][0])
            .max()
            .unwrap_or(i32::MAX)
    }
}