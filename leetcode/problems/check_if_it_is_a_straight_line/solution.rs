impl Solution {
    pub fn check_straight_line(mut coordinates: Vec<Vec<i32>>) -> bool {
        if let Some(a) = coordinates.pop() {
            let x0 = a[0] as f64;
            let y0 = a[1] as f64;
            coordinates.into_iter()
                .map(|a| (a[0] as f64, a[1] as f64))
                .map(|(x, y)| (x-x0)/(y-y0))
                .map(|x| if x.abs() == f64::INFINITY {x.abs()} else {x})
                .collect::<Vec<f64>>()
                .windows(2)
                .all(|a| a[0]==a[1])
        } else {
            false
        }
    }
}