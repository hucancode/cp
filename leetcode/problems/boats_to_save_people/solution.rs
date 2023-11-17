use std::collections::VecDeque;
impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort();
        let mut people = VecDeque::from(people);
        let mut ret = 0;
        while people.len() > 0 {
            if people.back().unwrap() + people.front().unwrap() <= limit {
                people.pop_front();
            }
            people.pop_back();
            ret += 1;
        }
        ret
    }
}