use std::cmp::{min, max};

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let x: i32 = salary.iter().sum::<i32>() 
            - *salary.iter().min().unwrap_or(&0) 
            - *salary.iter().max().unwrap_or(&0);
        let n = salary.len() - 2;
        return x as f64 / n as f64;
    }
}