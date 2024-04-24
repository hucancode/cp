impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let n = n as usize;
        let mut f = vec![0,1,1];
        while n >= f.len() {
            let m = f.len();
            f.push(f[m-1]+f[m-2]+f[m-3]);
        }
        f[n]
    }
}
