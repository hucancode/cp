use std::collections::BinaryHeap;
use std::cmp::max;
impl Solution {
    pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
        let mut q = BinaryHeap::new();
        let mut n = heights.len();
        let mut i = 0;
        while i < n-1 {
            //println!("climb from {} to {}", heights[i], heights[i+1]);
            let delta = max(0, heights[i+1] - heights[i]);
            q.push(delta);
            bricks -= delta;
            while ladders > 0 && bricks < 0 && !q.is_empty() {
                let delta = q.pop().unwrap();
                bricks += delta;
                ladders -= 1;
                //println!("use ladder to climb previous building, reclaim {delta} bricks (current {bricks})");
            }
            if bricks < 0 {
                //println!("can't climb");
                break;
            }
            i += 1;
        }
        return i as i32;
    }
}