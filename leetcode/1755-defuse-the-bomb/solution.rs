impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n = code.len();
        let mut ret = vec![0;n];
        for i in 0..n {
            let range = if k >= 0 { (1..=k) } else { (k..=-1) };
            ret[i] = range
                .map(|j| (i+n+j as usize)%n)
                .map(|j| code[j])
                .fold(0, |acc, x| acc+x); 
        }
        ret
    }
}
