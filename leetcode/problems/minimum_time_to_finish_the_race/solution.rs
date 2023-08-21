use std::cmp::min;
impl Solution {
    pub fn minimum_finish_time(tires: Vec<Vec<i32>>, change_time: i32, num_laps: i32) -> i32 {
        let x = tires.iter().map(|a|a[0]).min().unwrap_or(0);
        let INF = x * num_laps + change_time *(num_laps - 1);
        let n = num_laps as usize;
        let mut f = vec![INF;n]; // f[i] = at lap i, what is the best finish time?
        for t in tires {
            let mut k = t[0] as u64;
            let mut cost = k;
            f[0] = min(f[0], cost as i32);
            for i in 1..n {
                k *= t[1] as u64;
                cost += k;
                if cost > INF as u64 {
                    break;
                }
                f[i] = min(f[i], cost as i32);
            }
        }
        //println!("f (optimized 1) = {f:?}");
        for i in 0..n {
            //println!("solve for lap #{i}, best till now = {}", g[i]);
            for j in 0..i {
                let change = f[i-1-j] + f[j] + change_time;
                let keep = f[i];
                //println!("try change at {j}, change score = {} + {} + {} = {change}, keep score = {keep}", f[i-1-j], f[j], change_time);
                f[i] = min(keep, change);
            }
        }
        //println!("f(optimized 2) = {f:?}");
        return f[n-1];
    }
}