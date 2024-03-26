impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum%k != 0 {
            return false;
        }
        let sum = sum/k;
        let n = nums.len();
        let m = 1<<n;
        let mut f = vec![0;m];
        let mut seed = Vec::new();
        for mask in 0..m {
            if f[mask] == sum {
                seed.push(mask);
                continue;
            }
            for i in (0..n).filter(|&i| mask & (1<<i) == 0) {
                f[mask | (1<<i)] = f[mask] + nums[i];
            }
        }
        let mut candidates: Vec<usize> = seed.clone();
        for j in 2..k as usize {
            candidates = candidates.into_iter()
                .map(|mask| {
                    seed.iter()
                        .filter(|&x| mask&x == 0)
                        .map(|&x| mask | x)
                        .collect::<Vec<usize>>()
                })
                .flatten()
                .collect();
        }
        !candidates.is_empty()
    }
}
