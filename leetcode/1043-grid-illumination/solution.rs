impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::{HashMap, HashSet};
        let mut lamp_by_xy: HashSet<(i32, i32)> = HashSet::new();
        let mut lamp_by_row: HashMap<i32, i32> = HashMap::new();
        let mut lamp_by_col: HashMap<i32, i32> = HashMap::new();
        let mut lamp_by_delta_xy: HashMap<i32, i32> = HashMap::new();
        let mut lamp_by_delta_mxy: HashMap<i32, i32> = HashMap::new();
        let mut ret = Vec::new();
        for l in lamps {
            let lx = l[0];
            let ly = l[1];
            if lamp_by_xy.insert((lx, ly)) {
                *lamp_by_row.entry(lx).or_default() += 1;
                *lamp_by_col.entry(ly).or_default() += 1;
                *lamp_by_delta_xy.entry(lx-ly).or_default() += 1;
                *lamp_by_delta_mxy.entry(-lx-ly).or_default() += 1;
            }
        }
        for q in queries {
            let qx = q[0];
            let qy = q[1];
            let lit = *lamp_by_row.entry(qx).or_default() > 0 ||
                *lamp_by_col.entry(qy).or_default() > 0 ||
                *lamp_by_delta_xy.entry(qx-qy).or_default() > 0 ||
                *lamp_by_delta_mxy.entry(-qx-qy).or_default() > 0;
            ret.push(if lit {1} else {0});
            for x in qx-1..=qx+1 {
                for y in qy-1..=qy+1 {
                    if lamp_by_xy.remove(&(x,y)) {
                        *lamp_by_row.entry(x).or_default() -= 1;
                        *lamp_by_col.entry(y).or_default() -= 1;
                        *lamp_by_delta_xy.entry(x-y).or_default() -= 1;
                        *lamp_by_delta_mxy.entry(-x-y).or_default() -= 1;
                    }
                }
            }
        }
        return ret;
    }
}
