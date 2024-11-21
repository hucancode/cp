#[derive(Clone)]
enum Cell {
    Guard(usize),
    Wall(usize),
}
impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Ordering;
        let n = n as usize;
        let m = m as usize;
        let mut cell_by_rows = vec![Vec::new();m];
        let mut cell_by_cols = vec![Vec::new();n];
        let cmp = |c: &Cell, i: &usize| {
            match *c {
                Cell::Guard(x) => x.cmp(i),
                Cell::Wall(x) => x.cmp(i),
            }
        };
        for guard in guards {
            let i = guard[0] as usize;
            let j = guard[1] as usize;
            let idx = cell_by_rows[i]
                .binary_search_by(|c| cmp(c, &j))
                .unwrap_err();
            cell_by_rows[i].insert(idx, Cell::Guard(j));
            let idx = cell_by_cols[j]
                .binary_search_by(|c| cmp(c, &i))
                .unwrap_err();
            cell_by_cols[j].insert(idx, Cell::Guard(i));
        }
        for wall in walls {
            let i = wall[0] as usize;
            let j = wall[1] as usize;
            let idx = cell_by_rows[i]
                .binary_search_by(|c| cmp(c, &j))
                .unwrap_err();
            cell_by_rows[i].insert(idx, Cell::Wall(j));
            let idx = cell_by_cols[j]
                .binary_search_by(|c| cmp(c, &i))
                .unwrap_err();
            cell_by_cols[j].insert(idx, Cell::Wall(i));
        }
        //println!("cell by rows {cell_by_rows:?}");
        //println!("cell by cols {cell_by_cols:?}");
        let is_occupied_or_guarded = |i:usize,j:usize| {
            match cell_by_rows[i].binary_search_by(|c| cmp(c, &j)) {
                Err(k) => {
                    match cell_by_rows[i].get(k) {
                        Some(Cell::Guard(_)) => return true,
                        _ => (),
                    }
                    match cell_by_rows[i].get(k-1) {
                        Some(Cell::Guard(_)) => return true,
                        _ => (),
                    }
                },
                _ => {
                    return true;
                }
            }
            match cell_by_cols[j].binary_search_by(|c| cmp(c, &i)) {
                Err(k) => {
                    match cell_by_cols[j].get(k) {
                        Some(Cell::Guard(_)) => return true,
                        _ => (),
                    }
                    match cell_by_cols[j].get(k-1) {
                        Some(Cell::Guard(_)) => return true,
                        _ => (),
                    }
                },
                _ => {
                    return true;
                }
            }
            return false;
        };
        let mut ret = m*n;
        for i in 0..m {
            for j in 0..n {
                if is_occupied_or_guarded(i,j) {
                    ret -= 1;
                }
            }
        }
        ret as i32
    }
}
