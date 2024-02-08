impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut points: Vec<(i32, i32)> = points.into_iter()
            .map(|a|(a[0], a[1]))
            .collect();
        points.sort_by(|(ax, ay), (bx, by)| if ax == bx { by.cmp(&ay)} else { ax.cmp(&bx) });
        let n = points.len();
        let mut ret = 0;
        for i in 0..n {
            let (ax, ay) = points[i];
            let mut x = i32::MIN;
            let mut y = i32::MIN;
            for j in (i+1)..n {
                let (bx, by) = points[j];
                if by > ay {
                    continue;
                }
                if by <= y {
                    continue;
                }
                y = by;
                if bx == x {
                    continue;
                }
                x = bx;
                ret += 1;
            }
        }
        ret
    }
}