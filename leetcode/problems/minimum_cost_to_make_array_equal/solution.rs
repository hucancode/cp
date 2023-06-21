impl Solution {
    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut task: Vec<usize> = (0..n).collect();
        task.sort_by(|&a,&b| nums[a].cmp(&nums[b]));
        let mut pl = vec![0;n];
        let mut pr = vec![0;n];
        let mut h = nums[task[0]] as i64;
        let mut w = cost[task[0]] as i64;
        for i in 1..n {
            let dy = nums[task[i]] as i64 - h;
            let dx = w;
            pl[i] = pl[i-1] + dy*dx;
            h = nums[task[i]] as i64;
            w += cost[task[i]] as i64;
        }
        h = nums[task[n-1]] as i64;
        w = cost[task[n-1]] as i64;
        for i in (0..n-1).rev() {
            let dy = h - nums[task[i]] as i64;
            let dx = w;
            pr[i] = pr[i+1] + dy*dx;
            h = nums[task[i]] as i64;
            w += cost[task[i]] as i64;
        }
        (0..n).map(|i| pl[i] + pr[i])
            .min()
            .unwrap_or(0)
    }
}