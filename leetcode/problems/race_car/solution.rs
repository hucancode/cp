use std::collections::HashSet;
impl Solution {
    pub fn racecar(target: i32) -> i32 {
        let mut q = Vec::new();
        let mut vis = HashSet::new();
        q.push((target, -1));
        let mut ret = 0;
        while !q.is_empty() {
            let mut next = Vec::new();
            while let Some((distance, speed)) = q.pop() {
                if distance == 0 {
                    return ret;
                }
                if distance.abs() > target {
                    continue;
                }
                if vis.contains(&(distance, speed)) {
                    continue;
                }
                vis.insert((distance, speed));
                next.push((distance + speed, speed*2));
                next.push((distance, -speed.signum()));
            }
            ret += 1;
            q = next;
        }
        return 3*target;
    }
}