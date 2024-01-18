impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut f1 = 1;
        let mut f2 = 1;
        for i in 1..n {
            let f3 = f1 + f2;
            f1 = f2;
            f2 = f3;
        }
        f2
    }
}