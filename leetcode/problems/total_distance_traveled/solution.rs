use std::cmp::min;
impl Solution {
    pub fn distance_traveled(mut main_tank: i32, mut additional_tank: i32) -> i32 {
        let mut ret = 0;
        while main_tank >= 5 {
            main_tank -= 5;
            ret += 50;
            if additional_tank >= 1 {
                main_tank += 1;
                additional_tank -= 1;
            }
        }
        ret += main_tank * 10;
        return ret;
    }
}