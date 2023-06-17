use std::cmp::min;
impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
        let n = arr1.len();
        arr2.sort();
        arr2.dedup();
        let m = arr2.len();
        // f[i][j] = can take at most i item from arr2, what is the minimum of arr1[j]
        // f[0] = cannot take anything from arr2, so f[0] = arr1
        let mut f = vec![vec![i32::MAX;n];m+1];
        for j in 0..n {
            if j == 0 || arr1[j] > f[0][j-1] {
                 f[0][j] = arr1[j]
            } else {
                break
            }
        }
        for i in 1..=m {
            f[i][0] = min(arr1[0], arr2[0]);
            for j in 1..n {
                let prev = f[i-1][j-1];
                let k = arr2.partition_point(|&x| x <= prev);
                if k < m {
                    f[i][j] = min(f[i][j], arr2[k]);
                }
                let prev = min(prev, f[i][j-1]);
                if arr1[j] > prev {
                    f[i][j] = min(f[i][j], arr1[j]);
                }
            }
        }
        //println!("{f:?}");
        f.iter()
            .position(|a| a[n-1] != i32::MAX)
            .map_or(-1, |x| x as i32)
    }
}