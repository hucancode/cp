impl Solution {
    pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
        use std::cmp::min;
        robot.sort();
        factory.sort_by(|a,b| a[0].cmp(&b[0]));
        let n = robot.len();
        let m = factory.len();
        let mut f = vec![vec![0;m+1];n+1];
        for i in 1..=n {
            f[i][0] = 200_000_000_000i64;
        }
        // f[i][j] = use j factories to repair i robots, what is the minimum cost
        for i in 1..=n {
            for j in 1..=m {
                let factory_cap = factory[j-1][1];
                let factory_pos = factory[j-1][0];
                let to_repair = min(i, factory_cap as usize);
                let mut cost = 0;
                f[i][j] = f[i][j-1];
                for k in 1..=to_repair {
                    cost += (robot[i-k] - factory_pos).abs() as i64;
                    f[i][j] = min(f[i][j], f[i-k][j-1]+cost);
                }
            }
        }
        // println!("robot: {robot:?}");
        // println!("factory: {factory:?}");
        // println!("f: {f:?}");
        f[n][m]
    }
}
