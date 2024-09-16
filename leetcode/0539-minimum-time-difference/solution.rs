impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        use std::cmp::min;
        let n = time_points.len();
        let mut time_points: Vec<i32> = time_points.into_iter()
            .map(|s| {
                let hhmm = s.split(':')
                    .map(|s|s.parse::<i32>().unwrap_or(0))
                    .collect::<Vec<i32>>();
                hhmm[0]*60 + hhmm[1]
            })
            .collect();
        time_points.sort();
        let delta = time_points.windows(2)
            .map(|a| a[1] - a[0])
            .min()
            .unwrap_or(0);
        let delta_tail = time_points[0] + 24*60 - time_points[n-1];
        return min(delta, delta_tail);
    }
}
