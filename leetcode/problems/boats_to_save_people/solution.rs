impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort_by(|a,b|b.cmp(a));
        let mut boat = Vec::new();
        let mut ret = 0;
        for x in people.iter() {
            if let Some(y) = boat.last() {
                if x + y <= limit {
                    boat.pop();
                    ret += 1;
                    continue;
                }
            }
            boat.push(*x);
        }
        ret += boat.len() as i32;
        return ret;
    }
}