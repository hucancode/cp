impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, mut stations: Vec<Vec<i32>>) -> i32 {
        use std::cmp::{min, max};
        stations.push(vec![target,0]);
        let n = stations.len();
        let mut m = n;
        let mut f = vec![vec![-1;n];n];
        f[0][0] = start_fuel - stations[0][0];
        for i in 0..n {
            let mut best = 0;
            for j in 0..m {
                if f[i][j] < best {
                    continue;
                }
                //println!("at station {i}(mile {}), stopped at {j} station before has {} gas left", stations[i][0], f[i][j]);
                for k in i+1..n {
                    let distance = stations[k][0] - stations[i][0];
                    let extra = stations[i][1];
                    let fuel = f[i][j] - distance;
                    if fuel + extra < 0 {
                        break;
                    }
                    f[k][j] = max(f[k][j], fuel);
                    f[k][j+1] = max(f[k][j+1], fuel + extra);
                    if k == n-1 {
                        m = min(m, j+1);
                    }
                }
                best = max(best, f[i][j]);
            }
        }
        //println!("{f:?}");
        for i in 0..n {
            if f[n-1][i] >= 0 {
                return i as i32;
            }
        }
        -1
    }
}
