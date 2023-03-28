use std::cmp::min;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let n = days.len();
        let mut f = vec![365000i32; n];
        for (i, day) in days.iter().enumerate() {
            for (j, cost) in costs.iter().enumerate() {
                let offset = match(j) {
                    0 => 1,
                    1 => 7,
                    2 => 30,
                    _ => 0,
                };
                let target = day - offset;
                let prev = match (days.binary_search(&target)) {
                    Ok(k) => *f.get(k).unwrap_or(&0),
                    Err(k) => if k>0 {*f.get(k-1).unwrap_or(&0)} else {0},
                };
                if let Some(x) = f.get_mut(i) {
                    *x = min(*x, cost + prev);
                }
            }
        }
        return *f.last().unwrap_or(&0);
    }
}