impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut i = 0;
        let n = 1<<16;
        while i <= n {
            let x = i*i;
            let y = c-x;
            if y < 0 {
                break;
            }
            let j = (y as f32).sqrt() as i32;
            if j*j == y || (j+1)*(j+1) == y {
                return true;
            }
            i += 1;
        }
        return false;
    }
}
