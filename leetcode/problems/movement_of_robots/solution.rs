impl Solution {
    pub fn sum_distance(nums: Vec<i32>, s: String, d: i32) -> i32 {
        let mut next: Vec<i64> = s.chars()
            .map(|c| if c == 'L' {-d} else {d})
            .zip(nums.into_iter())
            .map(|(d, x)| x as i64 + d as i64)
            .collect();
        next.sort();
        let n = next.len();
        next.into_iter()
            .enumerate()
            .map(|(i, x)| x*(2*i-n+1) as i64)
            .fold(0, |acc, x| (acc + x)%1000_000_007) as i32
    }
}