const INF: i64 = 1000_000_007;
impl Solution {
    fn build(n: usize) -> Vec<Vec<i64>> {
        let mut ret = vec![Vec::new();n];
        for i in 0..n {
            ret[i].resize(i+1, 1);
            for j in 1..i {
                ret[i][j] = ret[i-1][j-1] + ret[i-1][j];
                ret[i][j] %= INF;
            }
        }
        ret
    }
    fn query(nums: Vec<i32>, c: &Vec<Vec<i64>>) -> i64 {
        let n = nums.len();
        if n <= 2 {
            return 1;
        }
        let p = nums[0];
        let left: Vec<i32> = nums.clone()
            .into_iter()
            .skip(1)
            .filter(|&x| x < p)
            .collect();
        let right: Vec<i32> = nums.clone()
            .into_iter()
            .skip(1)
            .filter(|&x| x > p)
            .collect();
        let mut ret = c[n-1][left.len()];
        ret *= Self::query(left, c);
        ret %= INF;
        ret *= Self::query(right, c);
        ret %= INF;
        ret
    }
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let c = Self::build(nums.len());
        // c[n][k] = the number of way to select k item from pool size n
        (Self::query(nums, &c) - 1) as i32
    }
}