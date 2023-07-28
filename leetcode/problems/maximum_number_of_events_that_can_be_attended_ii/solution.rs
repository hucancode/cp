use std::cmp::max;
impl Solution {
    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        events.sort_by(|a, b| a[1].cmp(&b[1]));
        //println!("events: {events:?}");
        let n = events.len();
        let mut prefix = vec![0;n+1];
        for _ in 0..k {
            let prev = prefix.clone();
            for i in 1..=n {
                let target = events[i-1][0];
                let j = events.partition_point(|x| x[1] < target);
                let score = events[i-1][2] + prev[j];
                prefix[i] = max(prefix[i], max(prefix[i-1],score));
            }
            //println!("{prefix:?}");
        }
        prefix.into_iter().max().unwrap_or(0)
    }
}