use std::collections::BinaryHeap;
use std::cmp::max;
impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let n = arr.len();
        let mut f = vec![1;n];
        let mut q: BinaryHeap<(i32,usize)> = arr.iter()
            .enumerate()
            .map(|(i,&x)| (-x, i))
            .collect();
        while let Some((_, i)) = q.pop() {
            for j in (1..=d) {
                let j = i as i32 + j;
                if j as usize >= n || arr[j as usize] >= arr[i] {
                    break;
                }
                f[i] = max(f[i], f[j as usize] + 1);
            }
            for j in (-d..0).rev() {
                let j = i as i32 + j;
                if j < 0 || arr[j as usize] >= arr[i] {
                    break;
                }
                f[i] = max(f[i], f[j as usize] + 1);
            }
        }
        f.iter().fold(0,|acc,&x| max(acc,x))
    }
}