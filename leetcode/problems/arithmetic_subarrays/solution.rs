impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        l.into_iter()
            .zip(r.into_iter())
            .map(|(l,r)| {
                let n = r-l+1;
                let range = nums.iter().skip(l as usize).take(n as usize);
                let min = *range.clone().min().unwrap_or(&0);
                let max = *range.clone().max().unwrap_or(&0);
                if (max-min)%(n-1) != 0 {
                    return false;
                }
                let d = (max-min)/(n-1);
                if d == 0 {
                    return true;
                }
                let mut vis = vec![false;n as usize];
                range.clone()
                    .map(|x| x-min)
                    .filter(|x| x%d == 0)
                    .map(|x| (x/d) as usize)
                    .for_each(|x| vis[x] = true);
                vis.iter().all(|&x| x)
            })
            .collect()
    }
}