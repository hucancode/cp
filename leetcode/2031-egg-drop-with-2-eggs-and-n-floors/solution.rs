impl Solution {
    pub fn super_egg_drop(mut k: i32, mut n: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut f = vec![vec![0;k+1];n+1]; 
        // f[i][j] = given i floors, j eggs
        // egg is not guaranteed to break at floor #1 or floor #i
        // what is the answer
        for i in 1..=n {
            f[i][1] = i;
            for j in 2..=k {
                f[i][j] = f[i][j-1];
                //println!("solving {j} eggs, 1-{i} floors, for now we know that we have to try up to {} times", f[i][j]);
                let mut l = 1;
                let mut r = i;
                while l < r {
                    let m = (l+r+1)/2;
                    let egg_lost = f[m-1][j-1];
                    let egg_stay = f[i-m][j];
                    //println!("drop egg from floor #{m}, if the egg lost we have to try up to {egg_lost} more times, else {egg_stay} more");
                    f[i][j] = f[i][j].min(egg_lost.max(egg_stay) + 1);
                    if egg_lost > egg_stay {
                        r = m - 1;
                    } else {
                        l = m;
                    }
                }
                //println!("- solved {j} eggs, 1-{i} floors, we have to try up to {} times", f[i][j]);
            }
        }
        //println!("{f:?}");
        return f[n][k] as i32;
    }
    pub fn two_egg_drop(n: i32) -> i32 {
        Self::super_egg_drop(2, n)
    }
}

