impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n/2 {
            let mut x = i;
            let mut y = i;
            let m = n - i*2 - 1;
            let mut value = Vec::new();
            let mut pos = Vec::new();
            for _ in 0..m {
                pos.push((x, y));
                value.push(matrix[x][y]);
                y += 1;
            }
            for _ in 0..m {
                pos.push((x, y));
                value.push(matrix[x][y]);
                x += 1;
            }
            for _ in 0..m {
                pos.push((x, y));
                value.push(matrix[x][y]);
                y -= 1;
            }
            for _ in 0..m {
                pos.push((x, y));
                value.push(matrix[x][y]);
                x -= 1;
            }
            for (&v, &(x, y)) in value
                .iter()
                .zip(pos.iter()
                    .cycle()
                    .skip(m)) {
                matrix[x][y] = v;
            }
        }
    }
}