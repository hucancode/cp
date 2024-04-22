impl Solution {
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        use std::collections::VecDeque;
        let n = 7000;
        let mut vis_front = vec![false; n+1];
        let mut vis_back = vec![false; n+1];
        for i in forbidden {
            if i as usize > n {
                continue;
            }
            vis_front[i as usize] = true;
            vis_back[i as usize] = true;
        }
        let mut q = VecDeque::new();
        q.push_back((0,0,false));
        while let Some((i, cost, back)) = q.pop_front() {
            if i == x {
                return cost;
            }
            if i as usize > n {
                continue;
            }
            if (back && vis_back[i as usize]) || 
                (!back && vis_front[i as usize]) {
                continue;
            }
            if back {
                vis_back[i as usize] = true;
            } else {
                vis_front[i as usize] = true;
            }
            if i - b >= 0 && !back {
                q.push_back((i-b, cost+1, true));
            }
            q.push_back((i+a, cost+1, false));
        }
        -1
    }
}
