impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_by(|a,b|a[1].cmp(&b[1]));
        let n = pairs.len();
        let mut f = vec![1;n];
        for i in 1..n {
            f[i] = f[i-1];
            let target = pairs[i][0];
            let j = pairs.partition_point(|a| a[1] < target);
            if j == 0 {
                continue;
            }
            //println!("at {}, can connect to {}", i, j-1);
            f[i] = std::cmp::max(f[i], 1+f[j-1]);
        }
        return f[n-1];
    }
}