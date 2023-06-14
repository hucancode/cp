impl Solution {
    fn pow(x: f64, n: u32) -> f64 {
        match n {
            0 => 1.0,
            1 => x,
            n => {
                let k = Self::pow(x, n/2);
                k*k*Self::pow(x, n%2)
            }
        }
    }
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n < 0 {
            return 1.0/Self::pow(x, -n as u32);
        }
        return Self::pow(x, n as u32);
    }
}