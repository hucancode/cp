use std::cmp::min;
impl Solution {
    pub fn minimum_white_tiles(floor: String, num_carpets: i32, carpet_len: i32) -> i32 {
        let floor: Vec<i32> = floor.chars()
            .map(|c| if c=='1' {1} else {0})
            .collect();
        let k = carpet_len as usize;
        let n = floor.len();
        let m = num_carpets as usize;
        let mut f = vec![0;n+1];
        for j in 1..=n {
            f[j] = f[j-1] + floor[j-1];
        }
        for i in 0..m {
            let mut next = vec![0;n+1];
            for j in 1..=n {
                let cover = f[j-min(k,j)];
                let skip = next[j-1] + floor[j-1];
                next[j] = min(cover, skip);
            }
            f = next;
        }
        f[n]
    }
}