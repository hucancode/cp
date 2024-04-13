impl Solution {
    pub fn find_integers(mut n: i32) -> i32 {
        let mut f = vec![1,2]; // f(i) = how many way to pick valid number not larger than 2^i
        for i in 2..32 {
            f.push(f[i-1] + f[i-2]);
        }
        //println!("{f:?}");
        let mut ret = 1;
        for i in (0..32).rev() {
            let pattern = (n>>i) & 0b11;
            match pattern {
                0b11 => {
                    // can't pick 1 here, go to the next candidate not larger than n
                    // 11xxxxxxxxx -> 00111111111
                    n = (1<<i) - 1;
                }
                0b01 => {
                    // can pick 1 here
                    ret += f[i];
                }
                _ => {}
            }
        }
        return ret;
    }
}
