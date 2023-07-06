use std::cmp::Ordering;
use std::cmp::min;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let scodes: Vec<usize> = s.as_bytes().iter()
            .map(|&x| {
                if x >= 'a' as u8 {
                    return x - 'a' as u8 + 26;
                }
                return x - 'A' as u8;
            })
            .map(|x| x as usize)
            .collect();
        let tcodes: Vec<usize> = t.as_bytes().iter()
            .map(|&x| {
                if x >= 'a' as u8 {
                    return x - 'a' as u8 + 26;
                }
                return x - 'A' as u8;
            })
            .map(|x| x as usize)
            .collect();
        let mut targets = vec![0;52];
        for c in tcodes {
            targets[c] += 1;
        }
        let n = s.len();
        let mut prefix = vec![vec![0;n+1];52];
        for i in 0..52 {
            for j in 1..=n {
                prefix[i][j] = prefix[i][j-1] + 
                    if scodes[j-1] == i {1} else {0};
            }
        }
        let mut ret = "";
        for r in 1..=n {
            let mut l = r+1;
            for c in 0..52 {
                if targets[c] == 0 {
                    continue;
                }
                let x = prefix[c][r] - targets[c];
                let j = prefix[c].binary_search_by(
                    |&y| if y > x {Ordering::Greater} else {Ordering::Less})
                .unwrap_err();
                //println!("found {c} at {j}");
                if j == 0 {
                    l = r+1;
                    break;
                }
                l = min(l, j);
            }
            if l > r {
                continue;
            }
            //println!("found ALL at {l}, range = {l}~{r}");
            if ret.is_empty() || ret.len() > r-l {
                ret = s.get(l-1..r).unwrap();
            }
        }
        return ret.to_string();
    }
}