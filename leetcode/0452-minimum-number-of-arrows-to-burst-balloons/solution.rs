impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by(|a,b| a[0].cmp(&b[0]));
        //println!("{points:?}");
        let mut x = i32::MAX;
        let mut ret = 0;
        for r in points {
            let a = r[0];
            let b = r[1];
            if a > x {
                //println!("burst at {x}");
                ret += 1;
                x = i32::MAX;
            }
            x = std::cmp::min(x, b);
        }
        return ret + 1;
    }
}
