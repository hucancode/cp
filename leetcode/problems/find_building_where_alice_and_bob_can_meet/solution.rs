use std::cmp::{min, max, Reverse};
use std::collections::BinaryHeap;
impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = heights.len();
        let m = queries.len();
        let mut ans = vec![-1;m];
        let mut queries: Vec<(usize, usize, usize)> = queries.into_iter()
            .enumerate()
            .map(|(i,a)| (max(a[0], a[1]) as usize, min(a[0], a[1]) as usize, i))
            .collect();
        queries.sort_by(|(a, _, _),(b, _, _)| b.cmp(&a));
        let mut q = BinaryHeap::new();
        //println!("{queries:?}");
        for (i, &h) in heights.iter().enumerate() {
            //println!("check building {i} height {h}");
            while let Some(&(a,b,ans_idx)) = queries.last() {
                if i < a {
                    break;
                }
                if heights[b] < heights[a] || a == b {
                    ans[ans_idx] = a as i32;
                } else {
                    let required_height = max(heights[a], heights[b]);
                    //println!("check query {a}-{b}, required height = {required_height}");
                    q.push((Reverse(required_height), ans_idx));
                }
                queries.pop();
            }
            while let Some(&(Reverse(required_height), ans_idx)) = q.peek() {
                //println!("required height for {ans_idx} is {required_height}");
                if h <= required_height {
                    break;
                }
                //println!("required height {required_height} for {ans_idx} is satisfied");
                ans[ans_idx] = i as i32;
                q.pop();
            }
        }
        ans
    }
}