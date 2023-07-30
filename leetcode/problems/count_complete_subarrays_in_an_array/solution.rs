impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let MAX = *nums.iter().max().unwrap_or(&0) as usize;
        let mut f: Vec<Vec<i32>> = vec![vec![0;MAX+1];n+1];
        for i in 1..=n {
            for x in 1..=MAX {
                f[i][x] = f[i-1][x];
            }
            let x = nums[i-1] as usize;
            f[i][x] += 1;
        }
        let check = |i: usize,j: usize| {
            for x in 1..=MAX {
                let c: i32 = f[i][x] - f[j][x];
                if (f[n][x] == 0) != (c == 0) {
                    return false;
                }
            }
            return true;
        };
        //println!("{f:?}");
        let mut ret = 0;
        for i in 1..=n {
            for j in (0..i).rev() {
                if check(i,j) {
                    ret += j+1;
                    break;
                }
            }
        }
        return ret as i32;
    }
}