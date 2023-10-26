impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        arr.sort();
        let INF: i64 = 1000_000_007;
        let n = arr.len();
        let mut f = vec![1;n];
        for i in 1..n {
            let c = arr[i];
            for j in 0..i {
                let a = arr[j];
                if c%a != 0 {
                    continue;
                }
                let b = c/a;
                if let Ok(k) = arr.binary_search(&b) {
                    //println!("{} could be {}+{}", arr[i], arr[j], arr[k]);
                    f[i] += f[j]*f[k];
                    f[i] %= INF;
                }
            }
        }
        return f.iter().fold(0, |acc,x| (acc+x)%INF) as i32;
    }
}