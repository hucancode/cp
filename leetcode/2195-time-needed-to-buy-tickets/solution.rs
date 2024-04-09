impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        use std::cmp::min;
        let k = k as usize;
        let cap = tickets[k];
        tickets.into_iter()
            .enumerate()
            .map(|(i,x)| if i > k { min(x, cap-1) } else { min(x, cap) })
            .sum()
    }
}
