impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        use std::cmp::min;
        let k = k as usize;
        let n = s.len();
        let s: Vec<usize> = s.chars()
            .map(|c| c as usize - 'a' as usize)
            .collect();
        let mut count = vec![0;26];
        let mut counts = vec![count.clone()];
        for &c in s.iter() {
            count[c] += 1;
            counts.push(count.clone());
        }
        let mut f = vec![vec![n as i32;n+1];k+1];
        f[0][0] = 0;
        let mut occ = 0;
        let mut token = 0;
        for j in 1..=n {
            let c = s[j-1];
            if c == token {
                occ += 1;
            } else {
                token = c;
                occ = 1;
            }
            let cost = if occ <= 1 {1} else if occ < 10 {2} else if occ < 100 {3} else {4};
            f[0][j] = cost + f[0][j-occ];
        }
        //println!("{:?}", f[0]);
        for i in 1..=k {
            for j in 1..=n {
                f[i][j] = min(f[i][j], f[i-1][j-1]);
                let c = s[j-1];
                for x in (0..=j).rev() {
                    let occ = counts[j][c] - counts[x][c];
                    let deletion = j - x - occ;
                    if deletion > i {
                        break;
                    }
                    // if occ > 1 {
                    //     println!("from {x} to {j}, char {} appeared {occ} times, need to delete {deletion} to connect them", (c as u8 + 'a' as u8) as char);
                    // }
                    let cost = if occ <= 1 {1} else if occ < 10 {2} else if occ < 100 {3} else {4};
                    f[i][j] = min(f[i][j], f[i-deletion][x] + cost);
                }
            }
        }
        return f[k][n];
    }
}
