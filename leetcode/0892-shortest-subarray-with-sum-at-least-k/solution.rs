impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::VecDeque;
        use std::cmp::min;
        let n = nums.len();
        let mut sum = 0;
        let mut cost = 0;
        let mut ret = n + 1;
        let mut prefix = vec![(0, 0)];
        let mut q = VecDeque::new();
        for i in 0..n {
            let mut x = nums[i];
            //println!("check {x}");
            sum += x;
            cost += 1;
            let mut acc = x;
            let mut count = 1;
            while acc <= 0 && !q.is_empty() {
                let (x, c) = q.pop_back().unwrap();
                acc += x;
                count += c;
            }
            if acc > 0 {
                q.push_back((acc, count));
            } else {
                sum = 0;
                cost = 0;
            }
            //println!("{q:?}");
            while sum >= k && !q.is_empty() {
                //println!("needs {cost} item to sum up to {sum}");
                ret = min(ret, cost);
                let (x, c) = q.pop_front().unwrap();
                sum -= x;
                cost -= c;
            }
        }
        if ret > n { -1 } else { ret as i32 }
    }
}
