impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        use std::cmp::min;
        let k = k as usize;
        let n = stones.len();
        if (n-1)%(k-1) != 0 {
            return -1;
        }
        let mut prefix = vec![0;n+1];
        for i in 1..=n {
            prefix[i] = prefix[i-1] + stones[i-1];
        }
        let mut f = vec![vec![1000_000_000;n+1];n];
        // f[i][j] = minimum cost to merge j stones from starting at stone i 
        for i in 0..n {
            f[i][1] = 0;
        }
        let mut max_jump = 1;
        while max_jump <= n {
            for i in 0..n {
                let mut q = vec![(i, 0, 0)];
                while let Some((j, stone_count, cost)) = q.pop() {
                    if stone_count == k {
                        f[i][j-i] = min(f[i][j-i], cost);
                        continue;
                    }
                    let mut jump = 1;
                    while jump <= max_jump && j + jump <= n {
                        let extra = prefix[j+jump] - prefix[j] + f[j][jump];
                        q.push((j+jump, stone_count+1, cost+extra));
                        jump += k-1;
                    }
                }
            }
            max_jump += k-1;
        }
        f[0][n]
    }
}
