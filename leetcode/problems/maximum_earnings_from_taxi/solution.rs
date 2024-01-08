use std::cmp::max;
impl Solution {
    pub fn max_taxi_earnings(n: i32, mut rides: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        rides.sort_by(|a,b| a[1].cmp(&b[1]));
        let mut earning = vec![0;n+1];
        let mut ret = 0;
        let mut j = 1;
        let mut i = 0;
        while j <= n && i < rides.len() {
            earning[j] = max(earning[j], earning[j-1]);
            let start = rides[i][0] as usize;
            let end = rides[i][1] as usize;
            let tip = rides[i][2] as usize;
            if j < end {
                j += 1;
            } else {
                let x = end - start + tip;
                earning[j] = max(earning[j], x as i64 + earning[start]);
                ret = max(ret, earning[j]);
                //println!("checking customer {} at {}, earning {}", i, end, earning[end]);
                i += 1;
            }
        }
        //println!("{earning:?}");
        ret
    }
}