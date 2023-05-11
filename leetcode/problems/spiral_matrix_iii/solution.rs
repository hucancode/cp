use std::collections::VecDeque;
use std::cmp::{min, max};
impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let mut mat = VecDeque::new();
        mat.push_back(VecDeque::new());
        mat[0].push_back((r_start, c_start));
        ret.push(vec![r_start, c_start]);
        let mut step = 0;// 0 = right, 1 = down, 2 = left, 3 = up
        while mat.len() < rows as usize || mat[0].len() < cols as usize {
            match (step%4) {
                0 => for row in mat.iter_mut() {
                    if let Some(&(x, y)) = row.back() {
                        if (y >= cols -1) {
                            break;
                        }
                        row.push_back((x, y + 1));
                        ret.push(vec![x, y + 1]);
                    }
                },
                1 => if let Some(row) = mat.back() {
                    let mut row = row.clone();
                    let (x, y) = row[0];
                    if x < rows - 1 {
                        for (x, y) in row.iter_mut().rev() {
                            *x += 1;
                            ret.push(vec![*x, *y]);
                        }
                        mat.push_back(row);
                    }
                },
                2 => for row in mat.iter_mut().rev() {
                    if let Some(&(x, y)) = row.front() {
                        if (y <= 0) {
                            break;
                        }
                        row.push_front((x, y - 1));
                        ret.push(vec![x, y - 1]);
                    }
                },
                _ => if let Some(row) = mat.front() {
                    let mut row = row.clone();
                    let (x, y) = row[0];
                    if x > 0 {
                        for (x, y) in row.iter_mut() {
                            *x -= 1;
                            ret.push(vec![*x, *y]);
                        }
                        mat.push_front(row);
                    }
                },
            };
            step += 1;
        }
        ret
    }
}