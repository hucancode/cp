impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let da = a & !c;// where a need to cancel 1
        let db = b & !c;// where b need to cancel 1
        let dab = c & !(a|b);// where a or b need to flip to 1
        (da.count_ones() + db.count_ones() + dab.count_ones()) as i32
    }
}