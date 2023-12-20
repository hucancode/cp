use std::cmp::{min, max};
impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let mut x = i32::MAX;
        let mut y = i32::MAX;
        for mut p in prices {
            y = min(y, max(x, p));
            x = min(x, p);
        }
        if money >= x + y {
            money - x - y
        } else {
            money
        }
    }
}