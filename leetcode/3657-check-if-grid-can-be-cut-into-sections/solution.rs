impl Solution {
    pub fn check_valid_cuts(n: i32, rectangles: Vec<Vec<i32>>) -> bool {
        let cut = |i, arr: &Vec<(i32, i32)>| {
            let (_, mut end) = arr[i];
            for (i, &(l, r)) in arr.iter().enumerate().skip(i+1) {
                if l >= end {
                    return Some(i);
                }
                end = end.max(r);
            }
            None
        };
        let mut spanx: Vec<(i32, i32)> = rectangles.iter().map(|e| (e[0], e[2])).collect();
        spanx.sort_by(|(a,_),(b,_)|a.cmp(&b));
        let mut spany: Vec<(i32, i32)> = rectangles.iter().map(|e| (e[1], e[3])).collect();
        spany.sort_by(|(a,_),(b,_)|a.cmp(&b));

        if let Some(i) = cut(0, &spanx) {
            if let Some(j) = cut(i, &spanx) {
                // println!("{spanx:?}");
                // println!("cut x at {i}, {j}");
                return true;
            }
        }
        if let Some(i) = cut(0, &spany) {
            if let Some(j) = cut(i, &spany) {
                // println!("{spany:?}");
                // println!("cut y at {i}, {j}");
                return true;
            }
        }
        return false;
    }
}
