impl Solution {
    pub fn find_permutation(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut n = nums.len();
        let mut best = i32::MAX;
        let mut ret = Vec::new();
        let mut scores = HashMap::new();
        let mut q = vec![(0,0)];
        let mut arr = Vec::new();
        let mut mask = 0;
        while let Some((tail, score)) = q.pop() {
            if score < 0 {
                if let Some(x) = arr.pop() {
                    mask ^= 1<<x;
                }
                continue;
            }
            if score >= best || 
                scores.get(&(tail, mask))
                    .is_some_and(|&x| x <= score) {
                continue;
            }
            //println!("insert {j} to {arr:?}\n{mask:#14b}");
            if arr.len() == n {
                //println!("arr {arr:?}, score {score}");
                best = score;
                ret = arr.clone();
                continue;
            }
            scores.insert((tail, mask), score);
            q.push((tail,-1));
            arr.push(tail as i32);
            mask |= 1<<tail;
            if arr.len() == n {
                let extra = (tail as i32 - nums[0]).abs();
                q.push((0, score + extra));
            } else {
                for x in (0..n).rev() {
                    if (1<<x) & mask == 0 {
                        let extra = (tail as i32 - nums[x]).abs();
                        q.push((x, score + extra));
                    }
                }
            }
        }
        return ret;
    }
}
