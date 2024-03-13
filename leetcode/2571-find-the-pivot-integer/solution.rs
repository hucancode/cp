impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let x_squared = n*(n+1)/2;
        let x = (x_squared as f32).sqrt() as i32;
        if x*x == x_squared {
            x
        } else {
            -1
        }
    }
}
